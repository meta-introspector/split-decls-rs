use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > Default for & mut IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw_mut (Default :: default ()) } }
}