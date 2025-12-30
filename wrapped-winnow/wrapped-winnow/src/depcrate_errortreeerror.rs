// Generated macro for TreeError (enum)
macro_rules! Depcrate_errorTreeError {
() => {
// Module: crate::error
// Provides: {"TreeError"}
// Dependencies: {}
# [doc = " Trace all error paths, particularly for tests"] # [derive (Debug)] # [cfg (feature = "std")] pub enum TreeError < I , C = StrContext > { # [doc = " Initial error that kicked things off"] Base (TreeErrorBase < I >) , # [doc = " Traces added to the error while walking back up the stack"] Stack { # [doc = " Initial error that kicked things off"] base : Box < Self > , # [doc = " Traces added to the error while walking back up the stack"] stack : Vec < TreeErrorFrame < I , C > > , } , # [doc = " All failed branches of an `alt`"] Alt (Vec < Self >) , }
};
}
