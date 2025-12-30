// Generated macro for Status (enum)
macro_rules! Depcrate_streamStatus {
() => {
// Module: crate::stream
// Provides: {"Status"}
// Dependencies: {}
# [doc = " Return value of a `process` operation."] # [derive (Debug , Copy , Clone , PartialEq)] pub enum Status { # [doc = " Operation completed successfully."] Ok , # [doc = " End of stream was reached."] # [doc = ""] # [doc = " When encoding, this means that a sync/full flush or `Finish` was"] # [doc = " completed. When decoding, this indicates that all data was decoded"] # [doc = " successfully."] StreamEnd , # [doc = " If the TELL_ANY_CHECK flags is specified when constructing a decoder,"] # [doc = " this informs that the `check` method will now return the underlying"] # [doc = " integrity check algorithm."] GetCheck , # [doc = " An error has not been encountered, but no progress is possible."] # [doc = ""] # [doc = " Processing can be continued normally by providing more input and/or more"] # [doc = " output space, if possible."] # [doc = ""] # [doc = " Typically the first call to `process` that can do no progress returns"] # [doc = " `Ok` instead of `MemNeeded`. Only the second consecutive call doing no"] # [doc = " progress will return `MemNeeded`."] MemNeeded , }
};
}
