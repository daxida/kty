Converts wiktionary data from https://kaikki.org/ to yomitan-compatible dictionaries.

This is a port of [kaikki-to-yomitan](https://github.com/yomidevs/kaikki-to-yomitan).

It offers no releases: every dictionary must be built locally. It is still a work in progress.

## How to run

This example use German (de) to English (en).

```
$ git clone https://github.com/daxida/kty
$ cargo install --path=kty
$ kty main de en
...
✓ Wrote yomitan dict @ data/dict/de/en/kty-de-en.zip (20.94 MB)
```

A list of supported languages isos can be found at `assets/language.json`

## Other options

Output of `kty main --help` (may be outdated):

```
Main dictionary

Usage: kty main [OPTIONS] <SOURCE> <TARGET> [DICT_NAME]

Arguments:
  <SOURCE>     Source language
  <TARGET>     Target language
  [DICT_NAME]  Dictionary name [default: kty]

Options:
  -k, --keep-files           Write intermediate files to disk
  -r, --redownload           Redownload kaikki files
      --skip-filter          Skip filtering the jsonl
  -v, --verbose              Verbose output
      --skip-tidy            Skip running tidy (IR generation)
      --skip-yomitan         Skip running yomitan (mainly for testing)
      --first <FIRST>        (debug) Only take the first n jsonlines before filtering. -1 for taking all jsonlines [default: -1]
      --filter <FILTER>      (debug) Only include entries matching certain key–value filters
      --reject <REJECT>      (debug) Exclude entries matching certain key–value filters
      --pretty               Write jsons with whitespace
      --root-dir <ROOT_DIR>  (test) Modify the root directory. For testing, set this to "tests" [default: data]
  -h, --help                 Print help
```

## Tests

Tests are run with `cargo test`. If you only want to run tests for the main dictionary in a single language pair, without capturing output:

```
cargo run -- main ja en --root-dir=tests --keep-files --pretty
```

To add a word to the testsuite, besides copy pasting it, you can run:

```
# If the target is English
cargo run --release -- main de en --skip-tidy --skip-yomitan --filter word,faul
cat data/kaikki/de-en-extract.tmp.jsonl >> tests/kaikki/de-en-extract.jsonl

# Otherwise
cargo run --release -- main de de --skip-tidy --skip-yomitan --filter word,faul
cat data/kaikki/de-de-extract.jsonl >> tests/kaikki/de-de-extract.jsonl
```
