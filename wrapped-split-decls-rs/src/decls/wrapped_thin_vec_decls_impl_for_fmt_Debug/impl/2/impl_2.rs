use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : fmt :: Debug > fmt :: Debug for Drain < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }