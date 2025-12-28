use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns true if `stream` is a TTY, and the current terminal"] # [doc = " [supports_hyperlinks]."] pub fn on (stream : Stream) -> bool { (std :: env :: var ("FORCE_HYPERLINK") . is_ok () || is_a_tty (stream)) && supports_hyperlinks () }
}