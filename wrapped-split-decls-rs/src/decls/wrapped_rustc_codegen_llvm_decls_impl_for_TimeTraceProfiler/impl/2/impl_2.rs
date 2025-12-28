use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }
}