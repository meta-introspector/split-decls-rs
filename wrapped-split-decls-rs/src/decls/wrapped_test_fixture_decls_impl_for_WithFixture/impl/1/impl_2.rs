use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < DB : ExpandDatabase + SourceDatabase + Default + 'static > WithFixture for DB { }