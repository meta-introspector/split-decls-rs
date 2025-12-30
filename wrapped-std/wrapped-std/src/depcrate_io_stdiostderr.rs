// Generated macro for Stderr (struct)
macro_rules! Depcrate_io_stdioStderr {
() => {
// Module: crate::io::stdio
// Provides: {"Stderr"}
// Dependencies: {}
# [doc = " A handle to the standard error stream of a process."] # [doc = ""] # [doc = " For more information, see the [`io::stderr`] method."] # [doc = ""] # [doc = " [`io::stderr`]: stderr"] # [doc = ""] # [doc = " ### Note: Windows Portability Considerations"] # [doc = ""] # [doc = " When operating in a console, the Windows implementation of this stream does not support"] # [doc = " non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return"] # [doc = " an error."] # [doc = ""] # [doc = " In a process with a detached console, such as one using"] # [doc = " `#![windows_subsystem = \"windows\"]`, or in a child process spawned from such a process,"] # [doc = " the contained handle will be null. In such cases, the standard library's `Read` and"] # [doc = " `Write` will do nothing and silently succeed. All other I/O operations, via the"] # [doc = " standard library or via raw Windows API calls, will fail."] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Stderr { inner : & 'static ReentrantLock < RefCell < StderrRaw > > , }
};
}
