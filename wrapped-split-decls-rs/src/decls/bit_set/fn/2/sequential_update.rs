use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn sequential_update < T : Idx > (mut self_update : impl FnMut (T) -> bool , it : impl Iterator < Item = T > ,) -> bool { it . fold (false , | changed , elem | self_update (elem) | changed) }