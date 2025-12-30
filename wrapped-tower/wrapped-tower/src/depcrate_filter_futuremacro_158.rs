// Generated macro for macro_158 (macro)
macro_rules! Depcrate_filter_futuremacro_158 {
() => {
// Module: crate::filter::future
// Provides: {"macro_158"}
// Dependencies: {}
pin_project ! { # [doc = " Filtered response future from [`AsyncFilter`] services."] # [doc = ""] # [doc = " [`AsyncFilter`]: crate::filter::AsyncFilter"] # [derive (Debug)] pub struct AsyncResponseFuture < P , S , Request > where P : AsyncPredicate < Request >, S : Service < P :: Request >, { # [pin] state : State < P :: Future , S :: Future >, service : S , } }
};
}
