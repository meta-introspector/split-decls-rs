use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct DependencyAnalysis { total_crates : usize , dependency_conflicts : Vec < ConflictReport > , dependency_types : HashMap < String , DependencyTypeStats > , }