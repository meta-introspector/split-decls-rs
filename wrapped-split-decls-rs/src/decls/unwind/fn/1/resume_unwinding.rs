use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub (super) fn resume_unwinding (payload : Box < dyn Any + Send >) -> ! { panic :: resume_unwind (payload) }
}