use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Box < Pat > > for Pat { fn from (value : Box < Pat >) -> Self { * value } }