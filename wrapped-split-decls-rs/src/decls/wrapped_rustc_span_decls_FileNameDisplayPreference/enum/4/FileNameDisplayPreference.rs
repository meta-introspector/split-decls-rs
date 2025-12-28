use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , Eq , PartialEq , Hash , Debug)] pub enum FileNameDisplayPreference { # [doc = " Display the path after the application of rewrite rules provided via `--remap-path-prefix`."] # [doc = " This is appropriate for paths that get embedded into files produced by the compiler."] Remapped , # [doc = " Display the path before the application of rewrite rules provided via `--remap-path-prefix`."] # [doc = " This is appropriate for use in user-facing output (such as diagnostics)."] Local , # [doc = " Display only the filename, as a way to reduce the verbosity of the output."] # [doc = " This is appropriate for use in user-facing output (such as diagnostics)."] Short , }
}