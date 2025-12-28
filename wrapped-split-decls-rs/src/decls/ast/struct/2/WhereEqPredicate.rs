use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An equality predicate (unsupported)."] # [doc = ""] # [doc = " E.g., `T = int`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WhereEqPredicate { pub lhs_ty : Box < Ty > , pub rhs_ty : Box < Ty > , }