use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSend > DynSend for Lock < T > { }