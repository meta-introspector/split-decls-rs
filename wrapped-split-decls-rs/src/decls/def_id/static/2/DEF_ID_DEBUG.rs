use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
pub static DEF_ID_DEBUG : AtomicRef < fn (DefId , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
}