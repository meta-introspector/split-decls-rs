use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , V > SortedMap < K , V > { # [inline] pub const fn new () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }