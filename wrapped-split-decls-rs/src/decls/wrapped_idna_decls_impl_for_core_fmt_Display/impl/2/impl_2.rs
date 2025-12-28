use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl core :: fmt :: Display for Errors { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (self , f) } }