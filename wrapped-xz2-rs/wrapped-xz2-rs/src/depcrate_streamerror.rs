// Generated macro for Error (enum)
macro_rules! Depcrate_streamError {
() => {
// Module: crate::stream
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Possible error codes that can be returned from a processing operation."] # [derive (Debug , Clone , PartialEq)] pub enum Error { # [doc = " The underlying data was corrupt."] Data , # [doc = " Invalid or unsupported options were specified."] Options , # [doc = " File format wasn't recognized."] Format , # [doc = " Memory usage limit was reached."] # [doc = ""] # [doc = " The memory limit can be increased with `set_memlimit`"] MemLimit , # [doc = " Memory couldn't be allocated."] Mem , # [doc = " A programming error was encountered."] Program , # [doc = " The `TELL_NO_CHECK` flag was specified and no integrity check was"] # [doc = " available for this stream."] NoCheck , # [doc = " The `TELL_UNSUPPORTED_CHECK` flag was specified and no integrity check"] # [doc = " isn't implemented in this build of liblzma for this stream."] UnsupportedCheck , }
};
}
