use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Get runtime address from loaded symbols"] fn get_runtime_address (symbol : & str) -> Result < String > { let output = Command :: new ("objdump") . arg ("-t") . arg ("/proc/self/exe") . output () ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; for line in stdout . lines () { if line . contains (symbol) && line . contains ("F .text") { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () >= 1 { return Ok (format ! ("0x{}" , parts [0])) ; } } } Err (anyhow :: anyhow ! ("Runtime symbol not found")) }
}