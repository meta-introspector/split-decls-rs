use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , PartialEq , Eq , Clone , Hash)] pub enum CountError { OutOfBounds , Misplaced , }
}