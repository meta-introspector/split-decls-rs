use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn get_declarations_by_module (module : & str) -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | { r . by_module . get (module) . map (| indices | { indices . iter () . filter_map (| & i | r . declarations . get (i) . cloned ()) . collect () }) . unwrap_or_default () }) . unwrap_or_default () }
}