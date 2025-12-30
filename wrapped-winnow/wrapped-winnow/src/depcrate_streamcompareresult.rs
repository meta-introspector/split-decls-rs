// Generated macro for CompareResult (enum)
macro_rules! Depcrate_streamCompareResult {
() => {
// Module: crate::stream
// Provides: {"CompareResult"}
// Dependencies: {}
# [doc = " Result of [`Compare::compare`]"] # [derive (Debug , Eq , PartialEq)] pub enum CompareResult { # [doc = " Comparison was successful"] # [doc = ""] # [doc = " `usize` is the end of the successful match within the buffer."] # [doc = " This is most relevant for caseless UTF-8 where `Compare::compare`'s parameter might be a different"] # [doc = " length than the match within the buffer."] Ok (usize) , # [doc = " We need more data to be sure"] Incomplete , # [doc = " Comparison failed"] Error , }
};
}
