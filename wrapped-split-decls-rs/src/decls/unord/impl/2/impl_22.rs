use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < V : Hash + Eq > Extend < V > for UnordSet < V > { # [inline] fn extend < T : IntoIterator < Item = V > > (& mut self , iter : T) { self . inner . extend (iter) } }