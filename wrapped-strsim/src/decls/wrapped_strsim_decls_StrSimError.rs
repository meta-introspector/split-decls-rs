use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}
