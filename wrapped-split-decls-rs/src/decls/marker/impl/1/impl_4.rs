use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }
}