use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > fmt :: Debug for SsoHashSet < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }