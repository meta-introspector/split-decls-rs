use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn test_process_crates_function () -> Result < () > { println ! ("Testing extracted process_crates_in_path function...") ; let test_path = Path :: new (".") ; let current_crate = "bootstrap3" ; let config = SplitDeclsConfig :: default () ; process_crates_in_path (test_path , current_crate , & config , false , true ,) ? ; println ! ("✅ Successfully called extracted function!") ; Ok (()) }