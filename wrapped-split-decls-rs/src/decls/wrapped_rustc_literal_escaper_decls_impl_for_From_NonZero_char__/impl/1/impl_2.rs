use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < NonZero < char > > for MixedUnit { # [inline] fn from (c : NonZero < char >) -> Self { MixedUnit :: Char (c) } }