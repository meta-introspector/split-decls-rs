use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: main");
fn main () -> Result < () > { let perf_file = "user_functions.txt" ; let output2_path = Path :: new ("output2") ; let functions = parse_perf_functions (perf_file) ? ; let bench_macros = generate_bench_macros (functions , output2_path) ; println ! ("// Generated perf2bench report") ; println ! ("// Maps perf hotspots to output2 macro declarations\n") ; for macro_call in bench_macros { match macro_call . wrap_path { Some (path) => { println ! ("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"{}\")" , macro_call . perf_id , macro_call . percentage , path) ; } None => { println ! ("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"NOT_FOUND\")" , macro_call . perf_id , macro_call . percentage) ; } } } Ok (()) }
}