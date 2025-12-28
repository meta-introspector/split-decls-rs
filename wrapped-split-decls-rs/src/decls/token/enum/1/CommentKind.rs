use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum CommentKind { Line , Block , }