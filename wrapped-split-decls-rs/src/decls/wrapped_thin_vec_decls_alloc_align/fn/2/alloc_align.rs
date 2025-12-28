use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Gets the align necessary to allocate a `ThinVec<T>`"] fn alloc_align < T > () -> usize { max (mem :: align_of :: < T > () , mem :: align_of :: < Header > ()) }
}