use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A draining iterator for `Slab`"] pub struct Drain < 'a , T > { inner : vec :: Drain < 'a , Entry < T > > , len : usize , }
}