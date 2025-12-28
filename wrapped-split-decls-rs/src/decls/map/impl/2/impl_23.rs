use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , K : Eq + Hash , V : Default > Entry < 'a , K , V > { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [inline] pub fn or_default (self) -> & 'a mut V { self . or_insert_with (Default :: default) } }