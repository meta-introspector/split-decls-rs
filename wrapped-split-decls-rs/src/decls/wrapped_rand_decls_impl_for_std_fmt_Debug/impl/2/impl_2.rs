use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "std")] impl < R : TryRngCore > std :: fmt :: Debug for RngReader < R > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("RngReader") . finish () } }