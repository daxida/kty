use anyhow::Result;
use std::fs;
use std::path::Path;

// Some pretty printing codepoints
pub const SKIP_C: &str = "⏭";
pub const CHECK_C: &str = "✓";

fn get_file_size_in_mb(path: &Path) -> Result<f64> {
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    Ok(size_mb)
}

fn pretty_msg_at_path(msg: &str, path: &Path) -> String {
    let at = "\x1b[1;36m@\x1b[0m"; // bold + cyan
    match get_file_size_in_mb(path) {
        core::result::Result::Ok(size_mb) => {
            let size_str = format!("\x1b[1m{size_mb:.2} MB\x1b[0m"); // bold
            format!("{msg} {at} {} ({})", path.display(), size_str)
        }
        // Happens when we write to zip
        Err(..) => format!("{msg} {at} {}", path.display()),
    }
}

pub fn pretty_println_at_path(msg: &str, path: &Path) {
    println!("{}", pretty_msg_at_path(msg, path));
}

pub fn pretty_print_at_path(msg: &str, path: &Path) {
    print!("{}", pretty_msg_at_path(msg, path));
}

pub fn skip_because_file_exists(skipped: &str, path: &Path) {
    let msg = format!("{SKIP_C} Skipping {skipped}: file already exists");
    pretty_println_at_path(&msg, path);
}
