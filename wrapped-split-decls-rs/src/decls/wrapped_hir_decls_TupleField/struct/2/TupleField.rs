use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub struct TupleField { pub owner : DefWithBodyId , pub tuple : TupleId , pub index : u32 , }
}