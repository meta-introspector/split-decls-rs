use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct BenchMacro { perf_id : String , percentage : f64 , wrap_path : Option < String > , }
}