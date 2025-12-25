use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn styled(ansi: bool, style: Style, text: impl AsRef<str>) -> String {
    if ansi {
        style.paint(text.as_ref()).to_string()
    } else {
        text.as_ref().to_string()
    }
}
