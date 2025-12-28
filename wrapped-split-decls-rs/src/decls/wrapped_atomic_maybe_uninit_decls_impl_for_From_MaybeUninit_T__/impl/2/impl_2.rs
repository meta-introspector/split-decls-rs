use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Primitive > From < MaybeUninit < T > > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from a potentially uninitialized value."] # [inline] fn from (v : MaybeUninit < T >) -> Self { Self :: new (v) } }
}