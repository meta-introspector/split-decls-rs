use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct ImplAdapter { pub trait_name : String , pub implementation_type : ImplementationType , pub mock_variant : bool , }
}