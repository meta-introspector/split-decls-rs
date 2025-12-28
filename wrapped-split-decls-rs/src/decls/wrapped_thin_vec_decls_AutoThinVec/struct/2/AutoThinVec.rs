use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc (hidden)] # [cfg (feature = "gecko-ffi")] # [repr (C)] pub struct AutoThinVec < T , const N : usize > { inner : ThinVec < T > , buffer : AutoBuffer < T , N > , _pinned : std :: marker :: PhantomPinned , }
}