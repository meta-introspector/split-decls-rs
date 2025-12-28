use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn test_input (g : LabelledGraph) -> io :: Result < String > { let mut writer = Vec :: new () ; render (& g , & mut writer) . unwrap () ; let mut s = String :: new () ; Read :: read_to_string (& mut & * writer , & mut s) ? ; Ok (s) }