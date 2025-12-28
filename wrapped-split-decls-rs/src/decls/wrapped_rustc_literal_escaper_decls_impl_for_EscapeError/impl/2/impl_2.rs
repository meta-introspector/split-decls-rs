use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl EscapeError { # [doc = " Returns true for actual errors, as opposed to warnings."] pub fn is_fatal (& self) -> bool { ! matches ! (self , EscapeError :: UnskippedWhitespaceWarning | EscapeError :: MultipleSkippedLinesWarning) } }
}