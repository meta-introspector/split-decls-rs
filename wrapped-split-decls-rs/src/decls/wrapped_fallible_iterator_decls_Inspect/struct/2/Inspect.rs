use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which passes each element to a closure before returning it."] # [derive (Clone , Debug)] pub struct Inspect < I , F > { it : I , f : F , }