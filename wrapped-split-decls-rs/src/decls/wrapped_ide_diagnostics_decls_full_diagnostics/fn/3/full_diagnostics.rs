use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Request both syntax and semantic diagnostics for the given [`FileId`]."] pub fn full_diagnostics (db : & RootDatabase , config : & DiagnosticsConfig , resolve : & AssistResolveStrategy , file_id : FileId ,) -> Vec < Diagnostic > { let mut res = syntax_diagnostics (db , config , file_id) ; let sema = semantic_diagnostics (db , config , resolve , file_id) ; res . extend (sema) ; res }
}