use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > Default for & IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw (Default :: default ()) } }