use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < RangeFrom < usize > > for TimesRange { fn from (r : RangeFrom < usize >) -> TimesRange { TimesRange (r . start .. usize :: MAX) } }