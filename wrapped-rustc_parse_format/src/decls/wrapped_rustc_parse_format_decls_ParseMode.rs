use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type of format string that we are parsing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseMode {
    /// A normal format string as per `format_args!`.
    Format,
    /// An inline assembly template string for `asm!`.
    InlineAsm,
    /// A format string for use in diagnostic attributes.
    ///
    /// Similar to `format_args!`, however only named ("captured") arguments
    /// are allowed, and no format modifiers are permitted.
    Diagnostic,
}
