use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub (super) enum JobResult < T > { None , Ok (T) , Panic (Box < dyn Any + Send >) , }
}