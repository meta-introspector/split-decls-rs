use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : fmt :: Debug > fmt :: Debug for WorkerLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WorkerLocal") . field ("registry" , & self . registry . id ()) . finish () } }
}