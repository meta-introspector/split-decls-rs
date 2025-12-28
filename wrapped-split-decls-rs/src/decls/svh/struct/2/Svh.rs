use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable_NoContext , Decodable_NoContext , Hash)] pub struct Svh { hash : Fingerprint , }
}