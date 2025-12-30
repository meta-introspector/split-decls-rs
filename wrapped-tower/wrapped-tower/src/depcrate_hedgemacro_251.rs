// Generated macro for macro_251 (macro)
macro_rules! Depcrate_hedgemacro_251 {
() => {
// Module: crate::hedge
// Provides: {"macro_251"}
// Dependencies: {}
pin_project ! { # [doc = " The [`Future`] returned by the [`Hedge`] service."] # [doc = ""] # [doc = " [`Future`]: std::future::Future"] # [derive (Debug)] pub struct Future < S , Request > where S : tower_service :: Service < Request >, { # [pin] inner : S :: Future , } }
};
}
