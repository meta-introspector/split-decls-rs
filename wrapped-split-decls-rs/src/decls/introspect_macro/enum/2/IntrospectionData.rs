use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum IntrospectionData { CompileTime (TokenStream) , Runtime (String) , Emergent (String) , }
}