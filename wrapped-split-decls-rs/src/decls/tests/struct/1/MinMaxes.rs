use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct MinMaxes (IndexVec < usize , MinMaxIn > , fn (usize) -> MinMaxIn) ;
}