use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MutTy { pub ty : Box < Ty > , pub mutbl : Mutability , }