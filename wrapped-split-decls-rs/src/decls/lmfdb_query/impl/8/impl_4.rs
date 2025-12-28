use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LMFDBQuery { pub fn from_k_node (level : u8 , complexity : f64 , depth : u32 , name : & str) -> Self { let mut query_params = HashMap :: new () ; let mut similarity_features = Vec :: new () ; let collection = match level { 7 => "elliptic_curves" , 4 => "number_fields" , 1 => "modular_forms" , _ => "lattices" } ; if complexity > 6.0 { query_params . insert ("conductor" . to_string () , Value :: Number (((complexity * 100.0) as i64) . into ())) ; similarity_features . push ("high_complexity_structure" . to_string ()) ; } if depth > 2 { query_params . insert ("degree" . to_string () , Value :: Number ((depth as i64) . into ())) ; similarity_features . push ("deep_dependency_pattern" . to_string ()) ; } if name . contains ("trait") { similarity_features . push ("algebraic_structure" . to_string ()) ; } if name . contains ("generator") { similarity_features . push ("generating_function" . to_string ()) ; } Self { collection : collection . to_string () , query_params , similarity_features , } } pub fn to_lmfdb_url (& self) -> String { let base_url = format ! ("https://www.lmfdb.org/{}" , self . collection) ; let params : Vec < String > = self . query_params . iter () . map (| (k , v) | format ! ("{}={}" , k , v)) . collect () ; if params . is_empty () { base_url } else { format ! ("{}?{}" , base_url , params . join ("&")) } } pub fn generate_llm_reflect_call (& self , k_addr : & str) -> String { format ! (r#"llm! {{
    reflect! {{
        k_node: "{}",
        lmfdb_collection: "{}",
        query_url: "{}",
        similarity_features: {:?},
        prompt: "Analyze the mathematical structure similarity between this K-theory dependency node and LMFDB objects. What algebraic patterns connect code complexity to mathematical invariants?"
    }}
}}"# , k_addr , self . collection , self . to_lmfdb_url () , self . similarity_features) } }
}