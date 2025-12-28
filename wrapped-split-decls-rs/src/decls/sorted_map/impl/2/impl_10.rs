use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V > Default for SortedMap < K , V > { # [inline] fn default () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }