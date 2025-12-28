use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Default for Scales { fn default () -> Self { Scales :: SI () } }