use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Maxes (IndexVec < usize , MaxReached > , fn (usize) -> usize) ;
}