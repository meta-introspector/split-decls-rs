use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : Idx > Iterator for MixedBitIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { match self { MixedBitIter :: Small (iter) => iter . next () , MixedBitIter :: Large (iter) => iter . next () , } } }