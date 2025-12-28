use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Default for Slab < T > { fn default () -> Self { Slab :: new () } }