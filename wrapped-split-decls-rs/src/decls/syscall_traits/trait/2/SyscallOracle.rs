use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Core trait for syscall auditing and safety"] pub trait SyscallOracle { fn audit_call (function_name : & str , syscall_type : & str) ; fn pre_call_hook (syscall_type : & str) ; fn post_call_hook (syscall_type : & str , result : & dyn std :: fmt :: Debug) ; }