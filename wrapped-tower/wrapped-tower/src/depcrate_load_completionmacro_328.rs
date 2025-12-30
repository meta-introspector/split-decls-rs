// Generated macro for macro_328 (macro)
macro_rules! Depcrate_load_completionmacro_328 {
() => {
// Module: crate::load::completion
// Provides: {"macro_328"}
// Dependencies: {}
pin_project ! { # [doc = " Attaches a `C`-typed completion tracker to the result of an `F`-typed [`Future`]."] # [derive (Debug)] pub struct TrackCompletionFuture < F , C , H > { # [pin] future : F , handle : Option < H >, completion : C , } }
};
}
