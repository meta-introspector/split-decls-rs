use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum PreciseCapturingArg { # [doc = " Lifetime parameter."] Lifetime (# [visitable (extra = LifetimeCtxt :: GenericArg)] Lifetime) , # [doc = " Type or const parameter."] Arg (Path , NodeId) , }