use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: main");
fn main () { println ! ("Testing CratePaths generation with output directory...") ; let crate_path = PathBuf :: from ("test_crate") ; let paths = setup_crate_paths (& crate_path) . unwrap () ; println ! ("Crate name: {}" , paths . crate_name) ; println ! ("Crate path: {}" , paths . crate_path . display ()) ; println ! ("Source files: {:?}" , paths . source_files . iter () . map (| p | p . display () . to_string ()) . collect ::< Vec < _ >> ()) ; println ! ("First source file: {:?}" , paths . source_files . first () . map (| p | p . display () . to_string ())) ; println ! ("Decls output dir: {}" , paths . decls_output_dir . display ()) ; let expected_output = PathBuf :: from ("output") . join ("test_crate") . join ("src") . join ("decls") ; assert_eq ! (paths . decls_output_dir , expected_output) ; println ! ("✅ Output directory path is correct: {}" , expected_output . display ()) ; println ! ("✅ Test passed!") ; }
}