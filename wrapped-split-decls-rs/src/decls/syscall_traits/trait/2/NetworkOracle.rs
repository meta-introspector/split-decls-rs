use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Network operations oracle"] pub trait NetworkOracle { fn audit_connect () -> Result < () , String > ; fn check_endpoint_safety (endpoint : & str) -> bool ; }
}