use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The kind of yield expression"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum YieldKind { # [doc = " yield expr { ... }"] Prefix (Option < Box < Expr > >) , # [doc = " expr.yield { ... }"] Postfix (Box < Expr >) , }