use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if the current terminal, detected through various environment
/// variables, is known to support unicode rendering.
pub fn supports_unicode() -> bool {
    if std::env::consts::OS == "windows" {
        std::env::var("CI").is_ok()
            || std::env::var("WT_SESSION").is_ok()
            || std::env::var("ConEmuTask") == Ok("{cmd:Cmder}".into())
            || std::env::var("TERM_PROGRAM") == Ok("vscode".into())
            || std::env::var("TERM") == Ok("xterm-256color".into())
            || std::env::var("TERM") == Ok("alacritty".into())
    } else if std::env::var("TERM") == Ok("linux".into()) {
        false
    } else {
        let ctype = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_CTYPE"))
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_else(|_| "".into())
            .to_uppercase();
        ctype.ends_with("UTF8") || ctype.ends_with("UTF-8")
    }
}
