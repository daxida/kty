Converts wiktionary data from [kaikki](https://kaikki.org/) ([wiktextract](https://github.com/tatuylonen/wiktextract)) to [yomitan](https://github.com/yomidevs/yomitan)-compatible dictionaries.

This is a port of [kaikki-to-yomitan](https://github.com/yomidevs/kaikki-to-yomitan).

Converted dictionaries can be found on the [downloads](https://daxida.github.io/kty/) page.

## Usage

This example use German (de) to English (en).

```console
$ cargo install --git https://github.com/daxida/kty
$ kty main de en
...
✓ Wrote yomitan dict @ data/dict/de/en/kty-de-en.zip (20.94 MB)
```

A list of supported languages isos can be found [here](https://daxida.github.io/kty/language/).

For more information, see the [documentation](https://daxida.github.io/kty).
