use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V > IntoIterator for SsoHashMap < K , V > { type IntoIter = Either < < ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , < FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter ()) , SsoHashMap :: Map (map) => Either :: Right (map . into_iter ()) , } } }