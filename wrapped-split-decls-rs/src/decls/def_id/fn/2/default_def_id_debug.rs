use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn default_def_id_debug (def_id : DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("krate" , & def_id . krate) . field ("index" , & def_id . index) . finish () }
}