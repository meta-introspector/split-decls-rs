use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > Default for IndexVec < I , T > { # [inline] fn default () -> Self { IndexVec :: new () } }