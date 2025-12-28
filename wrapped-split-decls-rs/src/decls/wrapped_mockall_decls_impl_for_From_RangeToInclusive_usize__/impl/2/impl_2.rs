use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < RangeToInclusive < usize > > for TimesRange { fn from (r : RangeToInclusive < usize >) -> TimesRange { TimesRange (0 .. r . end + 1) } }