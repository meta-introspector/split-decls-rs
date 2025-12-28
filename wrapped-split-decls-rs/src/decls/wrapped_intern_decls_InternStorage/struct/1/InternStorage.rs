use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct InternStorage < T : ? Sized > { map : OnceLock < InternMap < T > > , }