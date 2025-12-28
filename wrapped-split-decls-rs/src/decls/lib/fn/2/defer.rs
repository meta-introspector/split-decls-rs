use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns a structure that calls `f` when dropped."] pub fn defer < F : FnOnce () > (f : F) -> OnDrop < F > { OnDrop (Some (f)) }