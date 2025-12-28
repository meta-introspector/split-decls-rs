use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Display for FixedBitSet { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Binary :: fmt (& self , f) } }