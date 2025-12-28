use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () { let output = Command :: new ("ls") . arg ("-la") . output () . unwrap () ; println ! ("Files: {}" , String :: from_utf8_lossy (& output . stdout)) ; Command :: new ("echo") . arg ("hello") . status () . unwrap () ; }
}