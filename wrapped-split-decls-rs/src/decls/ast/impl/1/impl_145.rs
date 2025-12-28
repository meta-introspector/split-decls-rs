use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Box < Ty > > for Term { fn from (v : Box < Ty >) -> Self { Term :: Ty (v) } }