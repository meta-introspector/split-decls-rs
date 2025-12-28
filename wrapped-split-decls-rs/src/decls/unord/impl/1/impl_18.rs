use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < V : Eq + Hash > Default for UnordSet < V > { # [inline] fn default () -> Self { Self { inner : FxHashSet :: default () } } }