use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K : Hash + Eq , V , I : Iterator < Item = (K , V) > > From < UnordItems < (K , V) , I > > for UnordMap < K , V > { # [inline] fn from (items : UnordItems < (K , V) , I >) -> Self { UnordMap { inner : FxHashMap :: from_iter (items . 0) } } }