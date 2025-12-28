use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Macros20NormalizedIdent { # [inline] pub fn new (ident : Ident) -> Self { Macros20NormalizedIdent (ident . normalize_to_macros_2_0 ()) } pub fn with_dummy_span (name : Symbol) -> Self { Macros20NormalizedIdent (Ident :: with_dummy_span (name)) } }
}