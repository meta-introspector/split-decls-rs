use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Entry < T > { present : AtomicBool , value : UnsafeCell < MaybeUninit < T > > , }
}