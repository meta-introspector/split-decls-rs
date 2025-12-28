use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn flat_index (i : usize , j : usize , width : usize) -> usize { j * width + i }