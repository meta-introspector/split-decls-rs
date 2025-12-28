use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] pub struct TypeManifold { pub dimensions : [HashMap < String , f64 > ; 8] , pub relationships : HashMap < String , Vec < String > > , pub embeddings : HashMap < String , [f64 ; 8] > , }
}