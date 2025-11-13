Converts wiktionary data from https://kaikki.org/ to yomitan-compatible dictionaries.

This is a port of [kaikki-to-yomitan](https://github.com/yomidevs/kaikki-to-yomitan).

It offers no releases: every dictionary must be built locally. It is still a work in progress.

## How to run

This example use German (de) to English (en).

```
$ git clone https://github.com/daxida/kty
$ cargo install --path=kty
$ kty de en
...
✓ Wrote yomitan dict @ data/dict/de/en/kty.zip (20.94 MB)
```

A list of supported languages isos can be found at `assets/language.json`

## Other options

Output of `kty --help` (may be outdated):

```
Usage: kty [OPTIONS] <SOURCE> <TARGET> [DICT_NAME]

Arguments:
  <SOURCE>     Source language
  <TARGET>     Target language
  [DICT_NAME]  Dictionary name [default: kty]

Options:
      --delete-files         Delete temporary files
  -r, --redownload           Redownload kaikki files
      --skip-filter          Skip filtering the jsonl
      --skip-tidy            Skip running tidy (IR generation)
      --skip-yomitan         Skip running yomitan (mainly for testing)
      --first <FIRST>        (debug) Stop filtering after the nth jsonline. -1 for taking all entries [default: -1]
      --filter <FILTER>      (debug) Only include entries matching certain key–value filters
      --reject <REJECT>      (debug) Exclude entries matching certain key–value filters
      --ugly                 (debug) Write jsons without whitespace. Faster but unreadable
      --root-dir <ROOT_DIR>  (test) Modify the root directory. For testing, set this to "tests" [default: data]
  -h, --help                 Print help
  -V, --version              Print version
```
