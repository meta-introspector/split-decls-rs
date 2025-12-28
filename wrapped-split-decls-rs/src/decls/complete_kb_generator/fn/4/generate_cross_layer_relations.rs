use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn generate_cross_layer_relations (lmdfb : & serde_json :: Value) -> Result < String > { let mut content = String :: new () ; content . push_str ("# Cross-Layer Relationships\n") ; if let Some (nodes) = lmdfb ["nodes"] . as_object () { for (symbol , node) in nodes { if let Some (deps) = node . get ("dependencies") . and_then (| v | v . as_array ()) { for dep in deps { if let Some (dep_str) = dep . as_str () { let node_id = format ! ("rustc:{}" , sanitize_id (symbol)) ; let dep_id = format ! ("rustc:{}" , sanitize_id (dep_str)) ; content . push_str (& format ! ("{} lmdfb:dependsOn {} .\n" , node_id , dep_id)) ; } } } } } content . push_str ("\n") ; Ok (content) }
}