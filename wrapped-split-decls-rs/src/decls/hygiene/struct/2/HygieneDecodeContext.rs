use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Additional information used to assist in decoding hygiene data"] # [derive (Default)] pub struct HygieneDecodeContext { remapped_ctxts : Lock < IndexVec < u32 , Option < SyntaxContext > > > , }
}