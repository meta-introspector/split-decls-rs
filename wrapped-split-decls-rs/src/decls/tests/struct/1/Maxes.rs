use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Maxes (IndexVec < usize , MaxReached > , fn (usize) -> usize) ;