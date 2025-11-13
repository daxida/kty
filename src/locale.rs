use crate::lang::Lang;

pub fn get_locale_examples_string(target_iso: &Lang, n: usize) -> String {
    match target_iso {
        Lang::Fr => {
            if n == 1 {
                "1 exemple".to_string()
            } else {
                format!("{n} exemples")
            }
        }
        _ => {
            if n == 1 {
                "1 example".to_string()
            } else {
                format!("{n} examples")
            }
        }
    }
}
