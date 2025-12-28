use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > IntoIterator for SsoHashSet < T > { type IntoIter = std :: iter :: Map < < SsoHashMap < T , () > as IntoIterator > :: IntoIter , fn ((T , ())) -> T > ; type Item = < Self :: IntoIter as Iterator > :: Item ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . map . into_iter () . map (entry_to_key) } }