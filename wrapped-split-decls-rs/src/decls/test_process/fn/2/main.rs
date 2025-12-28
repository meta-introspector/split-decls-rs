use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () { let output = Command :: new ("ls") . arg ("-la") . output () . expect ("Failed to execute command") ; println ! ("Command output: {}" , String :: from_utf8_lossy (& output . stdout)) ; let result = std :: process :: Command :: new ("echo") . arg ("hello") . execute () ; }