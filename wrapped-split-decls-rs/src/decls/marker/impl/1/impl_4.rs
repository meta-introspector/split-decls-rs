use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }