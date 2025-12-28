use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn analyze_k_levels (graph : & DependencyGraph , decls : & [Declaration]) -> Result < KTheoryAnalysis > { println ! ("🧮 Computing K-theory levels 0-7...") ; let mut levels = Vec :: new () ; for k in 0 .. 8 { let level_analysis = compute_k_level (k , graph , decls) ? ; println ! ("   K{}: {} nodes, depth {}" , k , level_analysis . node_count , level_analysis . max_depth) ; levels . push (level_analysis) ; } Ok (KTheoryAnalysis { levels }) }
}