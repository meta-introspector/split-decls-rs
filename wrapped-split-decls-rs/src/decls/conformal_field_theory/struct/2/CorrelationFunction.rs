use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CorrelationFunction { pub fields : Vec < String > , pub value : f64 , pub conformal_invariant : bool , }
}