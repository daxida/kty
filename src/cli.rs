use anyhow::{Ok, Result, bail};
use clap::{Parser, Subcommand};
use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::lang::Lang;
use crate::models::WordEntry;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    // NOTE: the order in which this --verbose flag appears in subcommands help seems cursed.
    //
    /// Verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Main dictionary
    Main(Args),

    /// Short dictionary made from translations
    Glossary(SimpleArgs),

    // Phonetic transcription dictionary
    Ipa(SimpleArgs),
}

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[command(flatten)]
    pub lang: ArgsLang,

    /// Dictionary name
    #[arg(default_value = "kty")]
    pub dict_name: String,

    #[command(flatten)]
    pub options: ArgsOptions,

    // contains these extra skip parameters
    #[command(flatten)]
    pub skip: ArgsSkip,
}

#[derive(Parser, Debug, Default)]
pub struct SimpleArgs {
    #[command(flatten)]
    pub lang: ArgsLang,

    /// Dictionary name
    #[arg(default_value = "kty")]
    pub dict_name: String,

    #[command(flatten)]
    pub options: ArgsOptions,
}

#[derive(Parser, Debug, Default)]
pub struct ArgsLang {
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
}

#[derive(Parser, Debug, Default)]
pub struct ArgsOptions {
    // In the main dictionary, the filter file is always writen to disk, regardless of this.
    /// Write intermediate files to disk
    #[arg(long, short)]
    pub keep_files: bool,

    /// Redownload kaikki files
    #[arg(long, short)]
    pub redownload: bool,

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
    #[arg(long, value_parser = parse_tuple)]
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
    #[arg(long, value_parser = parse_tuple)]
    pub reject: Vec<(FilterKey, String)>,

    /// Write jsons with whitespace.
    #[arg(long)]
    pub pretty: bool,

    /// (test) Modify the root directory. For testing, set this to "tests"
    #[arg(long, default_value = "data")]
    pub root_dir: PathBuf,
}

/// Skip arguments. Only relevant for the main dictionary.
#[derive(Parser, Debug, Default)]
pub struct ArgsSkip {
    /// Skip filtering the jsonl
    #[arg(long = "skip-filtering", help_heading = "Skip")]
    pub filtering: bool,

    /// Skip running tidy (IR generation)
    #[arg(long = "skip-tidy", help_heading = "Skip")]
    pub tidy: bool,

    /// Skip running yomitan (mainly for testing)
    #[arg(long = "skip-yomitan", help_heading = "Skip")]
    pub yomitan: bool,
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

impl Cli {
    pub fn parse_cli() -> (Self, PathManager) {
        let mut cli = Self::parse();
        // we should be getting rid of edition at some point...
        let pm = match cli.command {
            Command::Main(ref mut args) => {
                args.lang.edition = args.lang.target;
                PathManager::from_args(DictionaryType::Main, args)
            }
            Command::Glossary(ref mut args) => {
                args.lang.edition = args.lang.target;
                PathManager::from_simple_args(DictionaryType::Glossary, args)
            }
            Command::Ipa(ref mut args) => {
                args.lang.edition = args.lang.target;
                PathManager::from_simple_args(DictionaryType::Ipa, args)
            }
        };
        (cli, pm)
    }
}

impl ArgsOptions {
    pub const fn has_filter_params(&self) -> bool {
        !self.filter.is_empty() || !self.reject.is_empty() || self.first != -1
    }
}

#[derive(Debug)]
pub enum DictionaryType {
    Main,
    Glossary,
    Ipa,
}

impl From<&Command> for DictionaryType {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::Main(_) => Self::Main,
            Command::Glossary(_) => Self::Glossary,
            Command::Ipa(_) => Self::Ipa,
        }
    }
}

impl fmt::Display for DictionaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => write!(f, "main"),
            Self::Glossary => write!(f, "glossary"),
            Self::Ipa => write!(f, "ipa"),
        }
    }
}

/// Helper struct to manage paths.
//
// It could have done directly with args, but tracking dict_ty is quite tricky. Also, this makes
// the intent of every call to either args or pm (PathManager) clearer. And better autocomplete!
#[derive(Debug)]
pub struct PathManager {
    dict_name: String,
    dict_ty: DictionaryType,

    edition: Lang,
    source: Lang,
    target: Lang,

    root_dir: PathBuf,
    keep_files: bool,
}

impl PathManager {
    pub fn new(
        dict_ty: DictionaryType,
        dict_name: &str,
        lang: &ArgsLang,
        options: &ArgsOptions,
    ) -> Self {
        Self {
            dict_name: dict_name.to_string(),
            dict_ty,
            edition: lang.edition,
            source: lang.source,
            target: lang.target,
            root_dir: options.root_dir.clone(),
            keep_files: options.keep_files,
        }
    }

    pub fn from_args(dict_ty: DictionaryType, args: &Args) -> Self {
        Self::new(dict_ty, &args.dict_name, &args.lang, &args.options)
    }

    pub fn from_simple_args(dict_ty: DictionaryType, args: &SimpleArgs) -> Self {
        Self::new(dict_ty, &args.dict_name, &args.lang, &args.options)
    }

    /// Example: `data/kaikki`
    fn dir_kaik(&self) -> PathBuf {
        self.root_dir.join("kaikki")
    }
    /// Example: `data/dict/el/el`
    fn dir_dict(&self) -> PathBuf {
        self.root_dir
            .join("dict")
            .join(format!("{}/{}", self.source, self.target))
    }
    /// Depends on the type of dictionary being made.
    ///
    /// Example: `data/dict/el/el/temp-main`
    /// Example: `data/dict/el/el/temp-glossary`
    fn dir_temp(&self) -> PathBuf {
        // Maybe remove the "temp-" altogether?
        self.dir_dict().join(format!("temp-{}", self.dict_ty))
    }
    /// Example: `data/dict/el/el/temp/tidy`
    fn dir_tidy(&self) -> PathBuf {
        self.dir_temp().join("tidy")
    }

    pub fn setup_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.dir_kaik())?;
        fs::create_dir_all(self.dir_dict())?;

        if self.keep_files {
            fs::create_dir_all(self.dir_tidy())?; // not needed for glossary
            fs::create_dir_all(self.dir_temp_dict())?;
        }

        Ok(())
    }

    /// Different in English and non-English editions. The English download is already filtered.
    ///
    /// Example (el):    `data/kaikki/el-extract.jsonl`
    /// Example (en-en): `data/kaikki/en-en-extract.jsonl`
    /// Example (de-en): `data/kaikki/de-en-extract.jsonl`
    pub fn path_jsonl_raw(&self) -> PathBuf {
        self.dir_kaik().join(match self.edition {
            Lang::En => format!("{}-{}-extract.jsonl", self.source, self.target),
            _ => format!("{}-extract.jsonl", self.edition),
        })
    }

    /// `data/kaikki/source-target.jsonl`
    ///
    /// Source and target are passed as arguments because some dictionaries may require a different
    /// combination in their input. F.e., the el-en glossary is made out of el-el-extract.jsonl
    ///
    /// Example (en-el): `data/kaikki/en-el-extract.jsonl`
    pub fn path_jsonl(&self, source: Lang, target: Lang) -> PathBuf {
        self.dir_kaik()
            .join(format!("{source}-{target}-extract.jsonl"))
    }

    /// `data/dict/source/target/temp/tidy/source-target-lemmas.json`
    ///
    /// Example: `data/dict/el/el/temp/tidy/el-el-lemmas.json`
    pub fn path_lemmas(&self) -> PathBuf {
        self.dir_tidy()
            .join(format!("{}-{}-lemmas.json", self.source, self.target))
    }

    /// `data/dict/source/target/temp/tidy/source-target-forms.json`
    ///
    /// Example: `data/dict/el/el/temp/tidy/el-el-forms.json`
    pub fn path_forms(&self) -> PathBuf {
        self.dir_tidy()
            .join(format!("{}-{}-forms.json", self.source, self.target))
    }

    /// Temporary working directory path used before zipping the dictionary.
    ///
    /// Example: `data/dict/el/el/temp/dict`
    pub fn dir_temp_dict(&self) -> PathBuf {
        self.dir_temp().join("dict")
    }

    // Should not go here, but since it uses dict_ty...
    // It exists so the dictionary index is in sync with PathManager::path_dict
    //
    /// Depends on the dictionary type (main, glossary etc.)
    ///
    /// Example: `dictionary_name-el-en`
    /// Example: `dictionary_name-el-en-gloss`
    pub fn dict_name_expanded(&self) -> String {
        match self.dict_ty {
            DictionaryType::Main => format!("{}-{}-{}", self.dict_name, self.source, self.target),
            DictionaryType::Glossary => {
                format!("{}-{}-{}-gloss", self.dict_name, self.source, self.target)
            }
            DictionaryType::Ipa => {
                format!("{}-{}-{}-ipa", self.dict_name, self.source, self.target)
            }
        }
    }

    /// Depends on the dictionary type (main, glossary etc.)
    ///
    /// Example: `data/dict/el/en/dictionary_name-el-en.zip`
    /// Example: `data/dict/el/en/dictionary_name-el-en-gloss.zip`
    pub fn path_dict(&self) -> PathBuf {
        self.dir_dict()
            .join(format!("{}.zip", self.dict_name_expanded()))
    }

    /// Example: `data/dict/el/el/temp/diagnostics`
    pub fn dir_diagnostics(&self) -> PathBuf {
        self.dir_temp().join("diagnostics")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_commands() {
        assert!(Cli::try_parse_from(["kty", "main", "el", "en"]).is_ok());
        assert!(Cli::try_parse_from(["kty", "glossary", "el", "en"]).is_ok());
    }

    #[test]
    fn main_needs_target_edition() {
        assert!(Cli::try_parse_from(["kty", "main", "grc", "el"]).is_ok());
        assert!(Cli::try_parse_from(["kty", "main", "el", "grc"]).is_err());
    }

    // #[test]
    // fn glossary_needs_source_edition() {
    //     assert!(Cli::try_parse_from(["kty", "glossary", "grc", "el"]).is_err());
    //     assert!(Cli::try_parse_from(["kty", "glossary", "el", "grc"]).is_ok());
    // }
    //
    // #[test]
    // fn glossary_can_not_be_monolingual() {
    //     assert!(Cli::try_parse_from(["kty", "glossary", "el", "el"]).is_err());
    // }

    #[test]
    fn filter_flag() {
        assert!(Args::try_parse_from(["_pname", "el", "el", "--filter", "foo,bar"]).is_err());
        assert!(Args::try_parse_from(["_pname", "el", "el", "--filter", "word,hello"]).is_ok());
        assert!(Args::try_parse_from(["_pname", "el", "el", "--reject", "pos,name"]).is_ok());
    }
}
