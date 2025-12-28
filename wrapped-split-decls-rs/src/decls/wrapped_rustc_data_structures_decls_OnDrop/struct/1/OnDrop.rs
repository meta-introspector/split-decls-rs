use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct OnDrop < F : FnOnce () > (Option < F >) ;