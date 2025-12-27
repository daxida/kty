use crate::lang::EditionLang;

// This should be done differently, and support every section of the dictionary (i.e. Etymology)

pub fn localize_examples_string(edition: EditionLang, n: usize) -> String {
    let (singular, plural) = match edition {
        EditionLang::Fr => ("exemple", "exemples"),
        EditionLang::De => ("Beispiel", "Beispiele"),
        EditionLang::Es => ("ejemplo", "ejemplos"),
        EditionLang::Ru => ("пример", "примеры"),
        EditionLang::Zh | EditionLang::Ja => return format!("{n} 例"), // special case
        _ => ("example", "examples"),
    };

    if n == 1 {
        format!("1 {singular}")
    } else {
        format!("{n} {plural}")
    }
}
