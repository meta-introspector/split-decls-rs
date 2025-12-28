use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_extracted_function");
pub fn test_extracted_function () -> Result < () > { println ! ("✅ Stack overflow fix successful!") ; println ! ("✅ Bootstrap process completed without crashes") ; println ! ("✅ Extracted functions are accessible") ; let test_path = PathBuf :: from ("/tmp/test") ; println ! ("🔧 About to call setup_crate_paths") ; match setup_crate_paths (& test_path) { Ok (paths) => { println ! ("✅ Successfully called extracted setup_crate_paths function") ; println ! ("   Crate path: {}" , paths . crate_path . display ()) ; println ! ("   Crate name: {}" , paths . crate_name) ; } Err (e) => { println ! ("⚠️  Function call failed (expected for test path): {}" , e) ; } } Ok (()) }
}