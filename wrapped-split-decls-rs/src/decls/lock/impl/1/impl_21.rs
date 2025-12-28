use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Default > Default for Lock < T > { # [inline] fn default () -> Self { Lock :: new (T :: default ()) } }