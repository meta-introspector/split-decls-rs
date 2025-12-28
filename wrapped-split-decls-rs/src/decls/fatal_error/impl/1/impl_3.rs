use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }