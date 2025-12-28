use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > IntoIterator for & 'a mut Slab < T > { type Item = (usize , & 'a mut T) ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }