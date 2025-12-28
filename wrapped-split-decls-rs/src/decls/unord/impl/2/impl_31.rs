use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K : Hash + Eq , V > FromIterator < (K , V) > for UnordMap < K , V > { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { UnordMap { inner : FxHashMap :: from_iter (iter) } } }