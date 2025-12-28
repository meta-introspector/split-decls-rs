use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S , R > InstrumentableService < R > for S where S : Service < R > + Sized { }
}