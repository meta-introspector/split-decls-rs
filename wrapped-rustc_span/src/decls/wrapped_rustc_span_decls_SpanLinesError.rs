use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SpanLinesError {
    DistinctSources(Box<DistinctSources>),
}
