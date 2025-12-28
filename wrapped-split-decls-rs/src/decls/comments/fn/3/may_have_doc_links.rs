use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A fast conservative estimate on whether the string can contain documentation links."] # [doc = " A pair of square brackets `[]` must exist in the string, but we only search for the"] # [doc = " opening bracket because brackets always go in pairs in practice."] # [inline] pub fn may_have_doc_links (s : & str) -> bool { s . contains ('[') }