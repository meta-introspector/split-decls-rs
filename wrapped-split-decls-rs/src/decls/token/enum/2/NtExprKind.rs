use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NtExprKind { Expr , Expr2021 { inferred : bool } , }
}