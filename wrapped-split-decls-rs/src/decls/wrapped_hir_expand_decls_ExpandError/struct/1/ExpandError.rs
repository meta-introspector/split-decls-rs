use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , PartialEq , Eq , Clone , Hash)] pub struct ExpandError { inner : Arc < (ExpandErrorKind , Span) > , }