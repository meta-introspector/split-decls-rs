use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn test_bootstrap3 () -> Result < () > { simple_test :: test_extracted_function () ? ; test_extracted_functions_v2 :: test_process_crates_function () ? ; function_caller :: call_all_functions () }