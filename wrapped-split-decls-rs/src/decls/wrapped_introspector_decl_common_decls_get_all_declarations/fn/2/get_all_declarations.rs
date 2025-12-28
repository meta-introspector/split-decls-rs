use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn get_all_declarations () -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | r . declarations . clone ()) . unwrap_or_default () }
}