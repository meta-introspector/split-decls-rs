use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn compute_ast_fingerprint (ast_representation : & str) -> String { log :: debug ! ("Computing AST fingerprint for AST representation: {}" , ast_representation) ; let mut hasher = Hasher :: new () ; hasher . update (ast_representation . as_bytes ()) ; let fingerprint = hasher . finalize () . to_hex () . to_string () ; log :: debug ! ("Computed fingerprint: {}" , fingerprint) ; fingerprint }
}