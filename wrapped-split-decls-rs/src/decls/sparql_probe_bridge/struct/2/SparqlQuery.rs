use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct SparqlQuery { pub name : String , pub query_type : QueryType , pub threshold : Option < f64 > , pub limit : Option < usize > , pub target_template : String , }
}