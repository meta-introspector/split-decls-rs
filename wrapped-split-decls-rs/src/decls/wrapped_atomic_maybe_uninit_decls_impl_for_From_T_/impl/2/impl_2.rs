use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Primitive > From < T > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from an initialized value."] # [inline] fn from (v : T) -> Self { Self :: new (MaybeUninit :: new (v)) } }
}