use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: get_nm_address");
fn get_nm_address (symbol : & str) -> Result < String > { let output = Command :: new ("nm") . arg ("-D") . arg ("/proc/self/exe") . output () ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; for line in stdout . lines () { if line . contains (symbol) { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () >= 1 && parts [0] != "0000000000000000" { return Ok (format ! ("0x{}" , parts [0])) ; } } } Err (anyhow :: anyhow ! ("Not found")) }
}