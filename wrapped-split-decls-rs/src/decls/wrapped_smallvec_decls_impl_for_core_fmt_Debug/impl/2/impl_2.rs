use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , I , const N : usize > core :: fmt :: Debug for Splice < 'a , I , N > where I : Debug + Iterator + 'a , < I as Iterator > :: Item : Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Splice") . field (& self . drain) . finish () } }