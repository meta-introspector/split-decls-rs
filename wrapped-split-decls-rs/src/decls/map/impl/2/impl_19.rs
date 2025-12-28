use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V > fmt :: Debug for SsoHashMap < K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }