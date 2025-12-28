use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The type of format string that we are parsing."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum ParseMode { # [doc = " A normal format string as per `format_args!`."] Format , # [doc = " An inline assembly template string for `asm!`."] InlineAsm , # [doc = " A format string for use in diagnostic attributes."] # [doc = ""] # [doc = " Similar to `format_args!`, however only named (\"captured\") arguments"] # [doc = " are allowed, and no format modifiers are permitted."] Diagnostic , }