use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Display for PackedFingerprint { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let copy = self . 0 ; copy . fmt (formatter) } }