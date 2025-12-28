use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Iterator > ExactSizeIterator for Splice < '_ , I > { }
}