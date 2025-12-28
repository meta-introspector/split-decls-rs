use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn dummy_spanned < T > (t : T) -> Spanned < T > { respan (DUMMY_SP , t) }