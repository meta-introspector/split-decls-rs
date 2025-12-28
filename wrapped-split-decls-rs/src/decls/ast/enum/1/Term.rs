use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum Term { Ty (Box < Ty >) , Const (AnonConst) , }