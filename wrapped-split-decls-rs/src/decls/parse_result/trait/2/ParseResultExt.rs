use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait ParseResultExt < T > : Sized { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static ; }
}