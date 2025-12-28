use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct MinMaxes (IndexVec < usize , MinMaxIn > , fn (usize) -> MinMaxIn) ;