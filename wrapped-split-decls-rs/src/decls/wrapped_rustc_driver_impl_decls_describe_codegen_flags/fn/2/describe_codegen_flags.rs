use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn describe_codegen_flags () { safe_println ! ("\nAvailable codegen options:\n") ; print_flag_list ("-C" , config :: CG_OPTIONS) ; }
}