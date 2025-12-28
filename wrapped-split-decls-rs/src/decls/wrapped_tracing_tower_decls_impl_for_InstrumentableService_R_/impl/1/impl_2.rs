use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S , R > InstrumentableService < R > for S where S : Service < R > + Sized { }