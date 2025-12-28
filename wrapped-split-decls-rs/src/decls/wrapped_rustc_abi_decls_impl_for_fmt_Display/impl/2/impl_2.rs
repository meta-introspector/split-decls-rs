use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for AlignFromBytesError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AlignFromBytesError :: NotPowerOfTwo (align) => { write ! (f , "`{align}` is not a power of 2") } AlignFromBytesError :: TooLarge (align) => write ! (f , "`{align}` is too large") , } } }