use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Hasher { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("crc32fast::Hasher") . finish () } }