use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait FlatMapInPlace < T > : Sized { fn flat_map_in_place < F , I > (& mut self , f : F) where F : FnMut (T) -> I , I : IntoIterator < Item = T > ; }