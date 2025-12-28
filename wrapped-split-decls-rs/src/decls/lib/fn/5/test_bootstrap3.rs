use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_bootstrap3");
pub fn test_bootstrap3 () -> Result < () > { simple_test :: test_extracted_function () ? ; test_extracted_functions_v2 :: test_process_crates_function () ? ; function_caller :: call_all_functions () ? ; test_tracing :: test_tracing () ? ; println ! ("Running simple splitter to generate output3...") ; let output = Command :: new ("cargo") . args (& ["run"]) . current_dir ("../simple-split") . output () ? ; if ! output . status . success () { eprintln ! ("Simple splitter failed: {}" , String :: from_utf8_lossy (& output . stderr)) ; return Err (anyhow :: anyhow ! ("Simple splitter execution failed")) ; } println ! ("Simple splitter completed successfully") ; Ok (()) }
}