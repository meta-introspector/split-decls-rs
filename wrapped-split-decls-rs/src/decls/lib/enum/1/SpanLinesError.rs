use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , PartialEq , Eq , Debug)] pub enum SpanLinesError { DistinctSources (Box < DistinctSources >) , }
}