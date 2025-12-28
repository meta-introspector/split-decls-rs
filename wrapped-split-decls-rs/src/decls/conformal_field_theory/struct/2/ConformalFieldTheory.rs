use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct ConformalFieldTheory { pub name : String , pub central_charge : f64 , pub primary_fields : Vec < PrimaryField > , pub correlation_functions : HashMap < String , CorrelationFunction > , }
}