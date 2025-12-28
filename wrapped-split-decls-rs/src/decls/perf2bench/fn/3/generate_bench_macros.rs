use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn generate_bench_macros (functions : Vec < PerfFunction > , output2_path : & Path) -> Vec < BenchMacro > { functions . into_iter () . map (| func | { let wrap_path = find_matching_decl (& func . function , output2_path) ; BenchMacro { perf_id : func . function . clone () , percentage : func . percentage , wrap_path , } }) . collect () }
}