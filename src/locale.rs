use crate::lang::Lang;

// This should be done differently, and support every section of the dictionary (i.e. Etymology)

pub fn get_locale_examples_string(target_iso: &Lang, n: usize) -> String {
    let (singular, plural) = match target_iso {
        Lang::Fr => ("exemple", "exemples"),
        Lang::De => ("Beispiel", "Beispiele"),
        Lang::Es => ("ejemplo", "ejemplos"),
        Lang::Ru => ("пример", "примеры"),
        Lang::Zh | Lang::Ja => return format!("{n} 例"), // special case
        _ => ("example", "examples"),
    };

    if n == 1 {
        format!("1 {singular}")
    } else {
        format!("{n} {plural}")
    }
}
