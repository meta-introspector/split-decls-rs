// Generated macro for rtabort (macro)
macro_rules! Depcrate_rtrtabort {
() => {
// Module: crate::rt
// Provides: {"rtabort"}
// Dependencies: {}
macro_rules ! rtabort { ($ ($ t : tt) *) => { { rtprintpanic ! ("fatal runtime error: {}, aborting\n" , format_args ! ($ ($ t) *)) ; crate :: process :: abort () ; } } }
};
}
