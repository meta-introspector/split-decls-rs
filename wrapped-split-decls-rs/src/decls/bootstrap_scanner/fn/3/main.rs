use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { println ! ("🚀 Bootstrap Duplicate Code Scanner") ; println ! ("Using loaded macros as search keys...\n") ; let mut scanner = DuplicateScanner :: new () ? ; scanner . scan_wrapped_output ("output2") ? ; scanner . generate_repl_commands () ? ; println ! ("\n✨ Scan complete! Use the generated REPL commands to explore duplicates.") ; Ok (()) }
}