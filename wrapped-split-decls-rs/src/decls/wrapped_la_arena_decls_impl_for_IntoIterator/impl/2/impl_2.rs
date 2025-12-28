use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > IntoIterator for Arena < T > { type Item = (Idx < T > , T) ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { IntoIter (self . data . into_iter () . enumerate ()) } }