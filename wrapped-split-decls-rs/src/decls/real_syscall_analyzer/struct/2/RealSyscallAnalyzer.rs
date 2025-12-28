use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct RealSyscallAnalyzer { pub syscall_patterns : HashMap < String , Vec < String > > , pub file_locations : HashMap < String , Vec < String > > , pub total_files_scanned : usize , }