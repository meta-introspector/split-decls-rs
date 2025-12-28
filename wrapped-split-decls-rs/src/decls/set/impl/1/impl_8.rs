use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Default for SsoHashSet < T > { # [inline] fn default () -> Self { Self :: new () } }