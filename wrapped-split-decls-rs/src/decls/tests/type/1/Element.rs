use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type Element = (usize , & 'static str) ;
}