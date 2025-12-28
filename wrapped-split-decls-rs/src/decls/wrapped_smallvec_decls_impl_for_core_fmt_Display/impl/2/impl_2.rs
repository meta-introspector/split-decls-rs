use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl core :: fmt :: Display for CollectionAllocErr { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Allocation error: {:?}" , self) } }