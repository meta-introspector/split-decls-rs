use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline (always)] fn swap < T , U > ((t , u) : (T , U)) -> (U , T) { (u , t) }