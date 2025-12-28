use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Get symbol address from debug symbols"] fn get_symbol_address (symbol : & str) -> Result < String > { let output = Command :: new ("nm") . arg ("-D") . arg ("/proc/self/exe") . output () ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; for line in stdout . lines () { if line . contains (symbol) { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () >= 3 { return Ok (format ! ("0x{}" , parts [0])) ; } } } Err (anyhow :: anyhow ! ("Symbol not found")) }
}