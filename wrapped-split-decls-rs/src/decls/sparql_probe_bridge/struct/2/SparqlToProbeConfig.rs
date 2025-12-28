use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct SparqlToProbeConfig { pub queries : Vec < SparqlQuery > , pub probe_templates : HashMap < String , ProbeTemplate > , }
}