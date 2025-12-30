use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: main");
fn main () -> Result < () , Box < dyn std :: error :: Error > > { let output3_dir = Path :: new ("../output3") ; let output4_dir = Path :: new ("../output4") ; println ! ("🔧 Bootstrap4: Proving ALL functions are wrapped!") ; println ! ("📂 Input:  {}" , output3_dir . display ()) ; println ! ("📂 Output: {}" , output4_dir . display ()) ; bootstrap_from_output3 (output3_dir , output4_dir) ? ; println ! ("✨ Bootstrap4 completed with ALL WRAPPED functions!") ; Ok (()) }
}