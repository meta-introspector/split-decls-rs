use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl < T : Sized > WithSubscriber for T { }
}