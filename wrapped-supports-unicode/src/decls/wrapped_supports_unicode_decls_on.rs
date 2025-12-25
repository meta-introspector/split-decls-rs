use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if `stream` is a TTY or the current terminal
/// [supports_unicode].
pub fn on(stream: Stream) -> bool {
    if !is_a_tty(stream) {
        true
    } else {
        supports_unicode()
    }
}
