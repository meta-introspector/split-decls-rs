// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl BufferedStandardStream { # [doc = " Create a new `BufferedStandardStream` with the given color preferences"] # [doc = " that writes to standard output via a buffered writer."] # [doc = ""] # [doc = " On Windows, if coloring is desired and a Windows console could not be"] # [doc = " found, then ANSI escape sequences are used instead."] # [doc = ""] # [doc = " The specific color/style settings can be configured when writing via"] # [doc = " the `WriteColor` trait."] pub fn stdout (choice : ColorChoice) -> BufferedStandardStream { let wtr = WriterInner :: create (StandardStreamType :: StdoutBuffered , choice) ; BufferedStandardStream { wtr : LossyStandardStream :: new (wtr) } } # [doc = " Create a new `BufferedStandardStream` with the given color preferences"] # [doc = " that writes to standard error via a buffered writer."] # [doc = ""] # [doc = " On Windows, if coloring is desired and a Windows console could not be"] # [doc = " found, then ANSI escape sequences are used instead."] # [doc = ""] # [doc = " The specific color/style settings can be configured when writing via"] # [doc = " the `WriteColor` trait."] pub fn stderr (choice : ColorChoice) -> BufferedStandardStream { let wtr = WriterInner :: create (StandardStreamType :: StderrBuffered , choice) ; BufferedStandardStream { wtr : LossyStandardStream :: new (wtr) } } }
};
}
