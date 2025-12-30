// Generated macro for Span (struct)
macro_rules! Depcrate_replaceSpan {
() => {
// Module: crate::replace
// Provides: {"Span"}
// Dependencies: {}
# [doc = " Data that should replace a particular range of the original."] # [derive (Clone)] struct Span { # [doc = " Span of the parent data to be replaced, inclusive of the start, exclusive of the end."] range : Range < usize > , # [doc = " New data to insert at the `start` position of the `original` data."] data : Rc < [u8] > , # [doc = " Whether this data is committed or provisional."] committed : bool , }
};
}
