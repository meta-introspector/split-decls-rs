use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Flags for incoming messages.
///
/// Flags provide additional information about incoming messages.
#[cfg(not(target_os = "redox"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RecvFlags(c_int);
