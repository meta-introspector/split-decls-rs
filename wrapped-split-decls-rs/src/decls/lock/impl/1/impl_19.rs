use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : DynSend > DynSync for Lock < T > { }
}