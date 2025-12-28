use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The layout of a type, alongside the type itself."] # [doc = " Provides various type traversal APIs (e.g., recursing into fields)."] # [doc = ""] # [doc = " Note that the layout is NOT guaranteed to always be identical"] # [doc = " to that obtained from `layout_of(ty)`, as we need to produce"] # [doc = " layouts for which Rust types do not exist, such as enum variants"] # [doc = " or synthetic fields of enums (i.e., discriminants) and wide pointers."] # [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable_Generic)] pub struct TyAndLayout < 'a , Ty > { pub ty : Ty , pub layout : Layout < 'a > , }
}