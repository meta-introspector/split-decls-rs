// Generated macro for StderrLock (struct)
macro_rules! Depcrate_io_stdioStderrLock {
() => {
// Module: crate::io::stdio
// Provides: {"StderrLock"}
// Dependencies: {}
# [doc = " A locked reference to the [`Stderr`] handle."] # [doc = ""] # [doc = " This handle implements the [`Write`] trait and is constructed via"] # [doc = " the [`Stderr::lock`] method. See its documentation for more."] # [doc = ""] # [doc = " ### Note: Windows Portability Considerations"] # [doc = ""] # [doc = " When operating in a console, the Windows implementation of this stream does not support"] # [doc = " non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return"] # [doc = " an error."] # [doc = ""] # [doc = " In a process with a detached console, such as one using"] # [doc = " `#![windows_subsystem = \"windows\"]`, or in a child process spawned from such a process,"] # [doc = " the contained handle will be null. In such cases, the standard library's `Read` and"] # [doc = " `Write` will do nothing and silently succeed. All other I/O operations, via the"] # [doc = " standard library or via raw Windows API calls, will fail."] # [must_use = "if unused stderr will immediately unlock"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct StderrLock < 'a > { inner : ReentrantLockGuard < 'a , RefCell < StderrRaw > > , }
};
}
