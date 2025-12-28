use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl core :: fmt :: Display for CapacityError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("insufficient capacity") } }
}