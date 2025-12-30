// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl StandardStream { # [doc = " Create a new `StandardStream` with the given color preferences that"] # [doc = " writes to standard output."] # [doc = ""] # [doc = " On Windows, if coloring is desired and a Windows console could not be"] # [doc = " found, then ANSI escape sequences are used instead."] # [doc = ""] # [doc = " The specific color/style settings can be configured when writing via"] # [doc = " the `WriteColor` trait."] pub fn stdout (choice : ColorChoice) -> StandardStream { let wtr = WriterInner :: create (StandardStreamType :: Stdout , choice) ; StandardStream { wtr : LossyStandardStream :: new (wtr) } } # [doc = " Create a new `StandardStream` with the given color preferences that"] # [doc = " writes to standard error."] # [doc = ""] # [doc = " On Windows, if coloring is desired and a Windows console could not be"] # [doc = " found, then ANSI escape sequences are used instead."] # [doc = ""] # [doc = " The specific color/style settings can be configured when writing via"] # [doc = " the `WriteColor` trait."] pub fn stderr (choice : ColorChoice) -> StandardStream { let wtr = WriterInner :: create (StandardStreamType :: Stderr , choice) ; StandardStream { wtr : LossyStandardStream :: new (wtr) } } # [doc = " Lock the underlying writer."] # [doc = ""] # [doc = " The lock guard returned also satisfies `io::Write` and"] # [doc = " `WriteColor`."] # [doc = ""] # [doc = " This method is **not reentrant**. It may panic if `lock` is called"] # [doc = " while a `StandardStreamLock` is still alive."] pub fn lock (& self) -> StandardStreamLock < '_ > { StandardStreamLock :: from_stream (self) } }
};
}
