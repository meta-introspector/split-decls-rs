use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSend > DynSync for Lock < T > { }