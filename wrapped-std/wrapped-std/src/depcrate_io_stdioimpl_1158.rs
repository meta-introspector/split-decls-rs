// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_io_stdioimpl_1158 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1158"}
// Dependencies: {}
impl Stdout { # [doc = " Locks this handle to the standard output stream, returning a writable"] # [doc = " guard."] # [doc = ""] # [doc = " The lock is released when the returned lock goes out of scope. The"] # [doc = " returned guard also implements the `Write` trait for writing data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{self, Write};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut stdout = io::stdout().lock();"] # [doc = ""] # [doc = "     stdout.write_all(b\"hello world\")?;"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn lock (& self) -> StdoutLock < 'static > { StdoutLock { inner : self . inner . lock () } } }
};
}
