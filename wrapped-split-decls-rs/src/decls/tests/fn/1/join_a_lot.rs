use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn join_a_lot (n : usize) { if n > 0 { join (| | join_a_lot (n - 1) , | | join_a_lot (n - 1)) ; } }