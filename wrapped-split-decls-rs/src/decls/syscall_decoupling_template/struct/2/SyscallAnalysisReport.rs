use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] pub struct SyscallAnalysisReport { pub filesystem_calls : usize , pub process_calls : usize , pub environment_calls : usize , pub io_calls : usize , pub network_calls : usize , pub libc_calls : usize , pub time_calls : usize , pub total_syscalls : usize , }