// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_io_stdioimpl_1171 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1171"}
// Dependencies: {}
impl Stderr { # [doc = " Locks this handle to the standard error stream, returning a writable"] # [doc = " guard."] # [doc = ""] # [doc = " The lock is released when the returned lock goes out of scope. The"] # [doc = " returned guard also implements the [`Write`] trait for writing data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::{self, Write};"] # [doc = ""] # [doc = " fn foo() -> io::Result<()> {"] # [doc = "     let stderr = io::stderr();"] # [doc = "     let mut handle = stderr.lock();"] # [doc = ""] # [doc = "     handle.write_all(b\"hello world\")?;"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn lock (& self) -> StderrLock < 'static > { StderrLock { inner : self . inner . lock () } } }
};
}
