use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [must_use] fn get_output (cmd : & str , args : & [& str]) -> Option < String > { let output = Command :: new (cmd) . args (args) . output () . ok () ? ; let mut stdout = output . status . success () . then_some (output . stdout) ? ; while stdout . last () . copied () == Some (b'\n') { stdout . pop () ; } String :: from_utf8 (stdout) . ok () }