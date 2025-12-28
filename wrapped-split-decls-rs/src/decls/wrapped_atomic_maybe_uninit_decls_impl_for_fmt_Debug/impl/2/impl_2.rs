use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Primitive > fmt :: Debug for AtomicMaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (core :: any :: type_name :: < Self > ()) } }