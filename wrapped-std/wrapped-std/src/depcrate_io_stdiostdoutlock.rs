// Generated macro for StdoutLock (struct)
macro_rules! Depcrate_io_stdioStdoutLock {
() => {
// Module: crate::io::stdio
// Provides: {"StdoutLock"}
// Dependencies: {}
# [doc = " A locked reference to the [`Stdout`] handle."] # [doc = ""] # [doc = " This handle implements the [`Write`] trait, and is constructed via"] # [doc = " the [`Stdout::lock`] method. See its documentation for more."] # [doc = ""] # [doc = " By default, the handle is line-buffered when connected to a terminal, meaning"] # [doc = " it flushes automatically when a newline (`\\n`) is encountered. For immediate"] # [doc = " output, you can manually call the [`flush`] method. When the handle goes out"] # [doc = " of scope, the buffer is automatically flushed."] # [doc = ""] # [doc = " ### Note: Windows Portability Considerations"] # [doc = ""] # [doc = " When operating in a console, the Windows implementation of this stream does not support"] # [doc = " non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return"] # [doc = " an error."] # [doc = ""] # [doc = " In a process with a detached console, such as one using"] # [doc = " `#![windows_subsystem = \"windows\"]`, or in a child process spawned from such a process,"] # [doc = " the contained handle will be null. In such cases, the standard library's `Read` and"] # [doc = " `Write` will do nothing and silently succeed. All other I/O operations, via the"] # [doc = " standard library or via raw Windows API calls, will fail."] # [doc = ""] # [doc = " [`flush`]: Write::flush"] # [must_use = "if unused stdout will immediately unlock"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct StdoutLock < 'a > { inner : ReentrantLockGuard < 'a , RefCell < LineWriter < StdoutRaw > > > , }
};
}
