use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Inner type of a field and type of wrapper."] # [derive (Copy , Clone)] pub (crate) enum FieldInnerTy < 'ty > { # [doc = " Field is wrapped in a `Option<$inner>`."] Option (& 'ty Type) , # [doc = " Field is wrapped in a `Vec<$inner>`."] Vec (& 'ty Type) , # [doc = " Field isn't wrapped in an outer type."] Plain (& 'ty Type) , }
}