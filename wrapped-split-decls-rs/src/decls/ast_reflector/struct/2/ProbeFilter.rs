use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProbeFilter { pub name_pattern : Option < String > , pub visibility : Option < String > , pub attributes : Vec < String > , pub contains_text : Option < String > , pub complexity_threshold : Option < f64 > , pub layer : Option < String > , }
}