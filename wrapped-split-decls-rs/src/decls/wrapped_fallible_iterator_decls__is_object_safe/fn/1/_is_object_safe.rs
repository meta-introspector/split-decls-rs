use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn _is_object_safe (_ : & dyn DoubleEndedFallibleIterator < Item = () , Error = () >) { }
}