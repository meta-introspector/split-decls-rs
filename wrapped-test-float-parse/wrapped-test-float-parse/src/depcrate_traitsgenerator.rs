// Generated macro for Generator (trait)
macro_rules! Depcrate_traitsGenerator {
() => {
// Module: crate::traits
// Provides: {"Generator"}
// Dependencies: {}
# [doc = " A test generator. Should provide an iterator that produces unique patterns to parse."] # [doc = ""] # [doc = " The iterator needs to provide a `WriteCtx` (could be anything), which is then used to"] # [doc = " write the string at a later step. This is done separately so that we can reuse string"] # [doc = " allocations (which otherwise turn out to be a pretty expensive part of these tests)."] pub trait Generator < F : Float > : Iterator < Item = Self :: WriteCtx > + Send + 'static { # [doc = " Full display and filtering name"] const NAME : & 'static str = Self :: SHORT_NAME ; # [doc = " Name for display with the progress bar"] const SHORT_NAME : & 'static str ; # [doc = " The context needed to create a test string."] type WriteCtx : Send ; # [doc = " Number of tests that will be run."] fn total_tests () -> u64 ; # [doc = " Constructor for this test generator."] fn new () -> Self ; # [doc = " Create a test string given write context, which was produced as a step from the iterator."] # [doc = ""] # [doc = " `s` will be provided empty."] fn write_string (s : & mut String , ctx : Self :: WriteCtx) ; }
};
}
