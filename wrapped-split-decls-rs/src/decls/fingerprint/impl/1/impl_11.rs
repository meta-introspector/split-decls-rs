use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Hash for Fingerprint { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { state . write_fingerprint (self) ; } }