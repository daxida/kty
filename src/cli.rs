use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::lang::Lang;
use crate::models::WordEntry;

#[derive(Parser, Debug, Default)]
#[command(version)]
pub struct Args {
    // We hide this for simplicity and because for our purposes, this is always equal to the target
    // language. We still keep this around in case it becomes useful later down the road.
    //
    // Internally, this is just set to target.
    //
    /// Edition language
    #[arg(skip)]
    pub edition: Lang,

    /// Source language
    pub source: Lang,

    /// Target language
    #[arg(value_parser = validate_edition)]
    pub target: Lang,

    /// Dictionary name
    #[arg(default_value = "kty")]
    pub dict_name: String,

    // The filter file is always writen to disk, regardless of this.
    /// Write intermediate files to disk
    #[arg(long, short)]
    pub keep_files: bool,

    /// Redownload kaikki files
    #[arg(long, short)]
    pub redownload: bool,

    /// Skip filtering the jsonl
    #[arg(long)]
    pub skip_filter: bool,

    /// Skip running tidy (IR generation)
    #[arg(long)]
    pub skip_tidy: bool,

    /// Skip running yomitan (mainly for testing)
    #[arg(long)]
    pub skip_yomitan: bool,

    /// (debug) Only take the first n jsonlines before filtering.
    /// -1 for taking all jsonlines
    #[arg(long, default_value_t = -1)]
    pub first: i32,

    // This filtering is done at filter_jsonl
    //
    // Example:
    //   `--filter pos,adv`
    //
    // You can specify this option multiple times:
    //   `--filter pos,adv --filter word,foo`
    //
    /// (debug) Only include entries matching certain key–value filters
    #[arg(long, value_parser = parse_tuple, conflicts_with = "skip_filter")]
    pub filter: Vec<(FilterKey, String)>,

    // This filtering is done at filter_jsonl
    //
    // Example:
    //   `--reject pos,adj`
    //
    // You can specify this option multiple times:
    //   `--reject pos,adj --reject word,foo`
    //
    /// (debug) Exclude entries matching certain key–value filters
    #[arg(long, value_parser = parse_tuple, conflicts_with = "skip_filter")]
    pub reject: Vec<(FilterKey, String)>,

    /// Write jsons with whitespace.
    #[arg(long)]
    pub pretty: bool,

    /// (test) Modify the root directory. For testing, set this to "tests"
    #[arg(long, default_value = "data")]
    pub root_dir: PathBuf,

    // If I ever decide on making this more powerful, these are good defaults:
    // https://github.com/astral-sh/ruff/blob/276f1d0d88d7815f70fabb712af44bb4de85d9a7/crates/ty/docs/tracing.md?plain=1#L19
    // https://github.com/astral-sh/ruff/blob/276f1d0d88d7815f70fabb712af44bb4de85d9a7/crates/ty/src/logging.rs
    /// Verbose output
    #[arg(long, short)]
    pub verbose: bool,
}

fn validate_edition(s: &str) -> Result<Lang, String> {
    let lang: Lang = s.parse().map_err(|e: String| e)?;
    if lang.has_edition() {
        core::result::Result::Ok(lang)
    } else {
        Err(format!(
            "{s} is not a language with an edition.\n{}",
            Lang::has_edition_help_message()
        ))
    }
}

fn parse_tuple(s: &str) -> Result<(FilterKey, String), String> {
    let parts: Vec<_> = s.split(',').map(|x| x.trim().to_string()).collect();
    if parts.len() != 2 {
        return Err("expected two comma-separated values".into());
    }
    let filter_key = FilterKey::try_from(parts[0].as_str()).map_err(|e| e.to_string())?;
    core::result::Result::Ok((filter_key, parts[1].clone()))
}

#[derive(Debug, Clone)]
pub enum FilterKey {
    LangCode,
    Word,
    Pos,
}

impl FilterKey {
    pub fn field_value<'a>(&self, entry: &'a WordEntry) -> &'a str {
        match self {
            Self::LangCode => &entry.lang_code,
            Self::Word => &entry.word,
            Self::Pos => &entry.pos,
        }
    }

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "lang_code" => Ok(Self::LangCode),
            "word" => Ok(Self::Word),
            "pos" => Ok(Self::Pos),
            other => bail!("unknown filter key '{other}'. Choose between: lang_code | word | pos",),
        }
    }
}

impl Args {
    pub fn parse_args() -> Self {
        let mut args = Self::parse();
        args.edition = args.target;
        args
    }

    pub fn set_edition(&mut self, lang: &str) -> Result<()> {
        let iso = validate_edition(lang).map_err(|e| anyhow!(e))?;
        self.edition = iso;
        Ok(())
    }

    pub fn set_source(&mut self, lang: &str) -> Result<()> {
        let iso = Lang::from_str(lang).map_err(|e| anyhow!(e))?;
        self.source = iso;
        Ok(())
    }

    pub fn set_target(&mut self, lang: &str) -> Result<()> {
        let iso = validate_edition(lang).map_err(|e| anyhow!(e))?;
        self.target = iso;
        Ok(())
    }

    pub fn set_dict_name(&mut self, dict_name: &str) {
        self.dict_name = dict_name.into();
    }

    /// Example: `data`
    pub fn set_root_dir(&mut self, new: &PathBuf) {
        self.root_dir = new.into();
    }

    /// Example: `data/kaikki`
    fn kaik_dir(&self) -> PathBuf {
        self.root_dir.join("kaikki")
    }
    /// Example: `data/dict`
    fn dict_dir(&self) -> PathBuf {
        self.root_dir.join("dict")
    }
    /// Example: `data/dict/el/el`
    fn pathdir_dict(&self) -> PathBuf {
        self.dict_dir()
            .join(format!("{}/{}", self.source, self.target))
    }
    /// Example: `data/dict/el/el/temp`
    pub fn temp_dir(&self) -> PathBuf {
        self.pathdir_dict().join("temp")
    }
    /// Example: `data/dict/el/el/temp/tidy`
    fn tidy_dir(&self) -> PathBuf {
        self.temp_dir().join("tidy")
    }

    pub fn setup_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.kaik_dir())?;
        fs::create_dir_all(self.pathdir_dict())?;

        if self.keep_files {
            fs::create_dir_all(self.tidy_dir())?;
            fs::create_dir_all(self.pathdir_dict_temp())?;
        }

        Ok(())
    }

    /// Different in English and non-English editions.
    ///
    /// Example (el):    `https://kaikki.org/elwiktionary/raw-wiktextract-data.jsonl.gz`
    /// Example (sh-en): `https://kaikki.org/dictionary/Serbo-Croatian/kaikki.org-dictionary-SerboCroatian.jsonl.gz`
    pub fn url_raw_jsonl_gz(&self) -> String {
        let root = "https://kaikki.org";

        match self.edition {
            // Default download name is: kaikki.org-dictionary-TARGET_LANGUAGE.jsonl.gz
            Lang::En => {
                let long = self.source.long();
                // Serbo-Croatian, Ancient Greek and such cases
                let language_no_special_chars: String = long
                    .chars()
                    .filter(|c| *c != ' ' && *c != '-')
                    .collect();
                let source_long_esc = long.replace(' ', "%20");
                format!(
                    "{root}/dictionary/{source_long_esc}/kaikki.org-dictionary-{language_no_special_chars}.jsonl.gz"
                )
            }
            // Default download name is: raw-wiktextract-data.jsonl.gz
            other => format!("{root}/{other}wiktionary/raw-wiktextract-data.jsonl.gz",),
        }
    }

    /// Different in English and non-English editions. The English download is already filtered.
    ///
    /// Example (el):    `data/kaikki/el-extract.jsonl`
    /// Example (en-en): `data/kaikki/en-en-extract.jsonl`
    /// Example (de-en): `data/kaikki/de-en-extract.jsonl`
    pub fn path_raw_jsonl(&self) -> PathBuf {
        self.kaik_dir().join(match self.edition {
            Lang::En => format!("{}-{}-extract.jsonl", self.source, self.target),
            _ => format!("{}-extract.jsonl", self.edition),
        })
    }

    /// `data/kaikki/source-target.jsonl`
    ///
    /// Example (en-el): `data/kaikki/en-el-extract.jsonl`
    pub fn path_jsonl(&self) -> PathBuf {
        self.kaik_dir()
            .join(format!("{}-{}-extract.jsonl", self.source, self.target))
    }

    /// `data/dict/source/target/temp/tidy/source-target-lemmas.json`
    ///
    /// Example: `data/dict/el/el/temp/tidy/el-el-lemmas.json`
    pub fn path_lemmas(&self) -> PathBuf {
        self.tidy_dir()
            .join(format!("{}-{}-lemmas.json", self.source, self.target))
    }

    /// `data/dict/source/target/temp/tidy/source-target-forms-0.json`
    ///
    /// Example: `data/dict/el/el/temp/tidy/el-el-forms-0.json`
    pub fn path_forms(&self) -> PathBuf {
        self.tidy_dir()
            .join(format!("{}-{}-forms-0.json", self.source, self.target))
    }

    /// Temporary working directory path used before zipping the dictionary.
    ///
    /// Example: `data/dict/el/el/temp/dict`
    pub fn pathdir_dict_temp(&self) -> PathBuf {
        self.temp_dir().join("dict")
    }

    /// Example: `data/dict/el/el/dictionary_name.zip`
    pub fn path_dict(&self) -> PathBuf {
        self.pathdir_dict().join(format!("{}.zip", self.dict_name))
    }

    // Assets paths

    fn pathdir_assets(&self) -> PathBuf {
        PathBuf::from("assets")
    }

    /// Example: `assets/styles.css`
    pub fn path_styles(&self) -> PathBuf {
        self.pathdir_assets().join("styles.css")
    }

    // Diagnostics paths

    /// Example: `data/dict/el/el/temp/diagnostics`
    pub fn pathdir_diagnostics(&self) -> PathBuf {
        self.temp_dir().join("diagnostics")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_inexistent_edition() {
        let mut args = Args::default();
        assert!(args.set_edition("grc").is_err());
        assert!(args.set_edition("grc").is_err());
    }

    #[test]
    fn test_filter_flag() {
        assert!(Args::try_parse_from(["kty", "el", "el", "--filter", "foo,bar"]).is_err());
        assert!(Args::try_parse_from(["kty", "el", "el", "--filter", "word,hello"]).is_ok());
        assert!(Args::try_parse_from(["kty", "el", "el", "--reject", "pos,name"]).is_ok());
        assert!(
            Args::try_parse_from(["kty", "el", "el", "--skip-filter", "--reject", "pos,name"])
                .is_err()
        );
    }
}
