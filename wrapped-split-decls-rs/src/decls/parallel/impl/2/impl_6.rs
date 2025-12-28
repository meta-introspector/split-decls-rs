use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ParallelGuard { pub fn run < R > (& self , f : impl FnOnce () -> R) -> Option < R > { catch_unwind (AssertUnwindSafe (f)) . map_err (| err | { let mut panic = self . panic . lock () ; if panic . is_none () || ! (* err) . is :: < FatalErrorMarker > () { * panic = Some (IntoDynSyncSend (err)) ; } }) . ok () } }