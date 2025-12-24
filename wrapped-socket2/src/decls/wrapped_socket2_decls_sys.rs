use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg_attr(unix, path = "sys/unix.rs")]
#[cfg_attr(windows, path = "sys/windows.rs")]
mod sys;
