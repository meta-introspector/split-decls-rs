// Generated macro for Utf16CharsError (struct)
macro_rules! Depcrate_reportUtf16CharsError {
() => {
// Module: crate::report
// Provides: {"Utf16CharsError"}
// Dependencies: {}
# [doc = " A type for signaling UTF-16 errors."] # [doc = ""] # [doc = " The value of the unpaired surrogate is not exposed in order"] # [doc = " to keep the `Result` type (and `Option`-wrapping thereof)"] # [doc = " the same size as `char`. See an [issue about the representation][1]."] # [doc = ""] # [doc = " Note: `core::error::Error` is not implemented due to implementing it"] # [doc = " being an [unstable feature][2] at the time of writing."] # [doc = ""] # [doc = " [1]: https://github.com/rust-lang/rust/issues/118367"] # [doc = " [2]: https://github.com/rust-lang/rust/issues/103765"] # [derive (Debug , PartialEq)] # [non_exhaustive] pub struct Utf16CharsError ;
};
}
