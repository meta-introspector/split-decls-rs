use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > IntoIterator for & 'a CfgOptions { type Item = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < & FxHashSet < CfgAtom > as IntoIterator > :: into_iter (& self . enabled) } }