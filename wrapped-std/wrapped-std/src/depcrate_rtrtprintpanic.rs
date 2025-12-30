// Generated macro for rtprintpanic (macro)
macro_rules! Depcrate_rtrtprintpanic {
() => {
// Module: crate::rt
// Provides: {"rtprintpanic"}
// Dependencies: {}
macro_rules ! rtprintpanic { ($ ($ t : tt) *) => { # [cfg (not (feature = "panic_immediate_abort"))] if let Some (mut out) = crate :: sys :: stdio :: panic_output () { let _ = crate :: io :: Write :: write_fmt (& mut out , format_args ! ($ ($ t) *)) ; } # [cfg (feature = "panic_immediate_abort")] { let _ = format_args ! ($ ($ t) *) ; } } }
};
}
