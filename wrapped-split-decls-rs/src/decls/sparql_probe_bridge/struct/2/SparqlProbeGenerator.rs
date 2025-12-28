use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct SparqlProbeGenerator { rdf_data : HashMap < String , f64 > , frequency_data : HashMap < String , usize > , }
}