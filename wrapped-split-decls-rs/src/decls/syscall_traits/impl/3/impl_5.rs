use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SyscallOracle for DefaultSyscallOracle { fn audit_call (function_name : & str , syscall_type : & str) { eprintln ! ("AUDIT: {} called syscall type: {}" , function_name , syscall_type) ; } fn pre_call_hook (syscall_type : & str) { eprintln ! ("PRE_HOOK: {}" , syscall_type) ; } fn post_call_hook (syscall_type : & str , result : & dyn std :: fmt :: Debug) { eprintln ! ("POST_HOOK: {} -> {:?}" , syscall_type , result) ; } }