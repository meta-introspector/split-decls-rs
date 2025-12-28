use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < H : HashStableContext > HashStable < H > for RelativeBytePos { fn hash_stable (& self , hcx : & mut H , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }