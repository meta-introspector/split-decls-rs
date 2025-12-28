use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn execute_lean4_proof () -> bool { use std :: process :: Command ; println ! ("\n⚡ STEP 3: LEAN4 EXECUTION") ; match Command :: new ("lean") . args (& ["--run" , "lean4_proof/Main.lean"]) . current_dir (".") . output () { Ok (output) => { if output . status . success () { println ! ("   ✅ Lean4 proof executed successfully!") ; if ! output . stdout . is_empty () { println ! ("   📤 Lean4 output:") ; println ! ("{}" , String :: from_utf8_lossy (& output . stdout)) ; } true } else { println ! ("   📝 Lean4 execution failed (proof files generated)") ; if ! output . stderr . is_empty () { println ! ("   ⚠️  Error: {}" , String :: from_utf8_lossy (& output . stderr)) ; } false } } Err (_) => { println ! ("   📝 Lean4 not available (proof files generated)") ; false } } }
}