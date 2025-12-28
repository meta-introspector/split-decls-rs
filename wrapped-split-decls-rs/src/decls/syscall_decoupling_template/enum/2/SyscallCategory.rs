use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub enum SyscallCategory { FileSystem , Process , Environment , IO , Network , Memory , Time , }