use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: discover_available_macros");
fn discover_available_macros () -> HashMap < String , MacroDefinition > { let mut macros = HashMap :: new () ; macros . insert ("mkbootstrap" . to_string () , MacroDefinition { name : "mkbootstrap" . to_string () , purpose : "Generate bootstrap infrastructure" . to_string () , confidence : 0.9 , }) ; macros . insert ("mkbuildrs" . to_string () , MacroDefinition { name : "mkbuildrs" . to_string () , purpose : "Generate self-contained build.rs" . to_string () , confidence : 0.8 , }) ; macros }
}