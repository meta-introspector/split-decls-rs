use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , P , T , HCX > HashStable < HCX > for TaggedRef < 'a , P , T > where P : HashStable < HCX > + Aligned + ? Sized , T : Tag + HashStable < HCX > , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . pointer () . hash_stable (hcx , hasher) ; self . tag () . hash_stable (hcx , hasher) ; } }