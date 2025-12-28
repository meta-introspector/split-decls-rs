use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Box < Ty > > for Ty { fn from (value : Box < Ty >) -> Self { * value } }