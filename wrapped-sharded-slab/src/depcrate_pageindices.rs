// Generated macro for indices (function)
macro_rules! Depcrate_pageindices {
() => {
// Module: crate::page
// Provides: {"indices"}
// Dependencies: {}
# [inline (always)] pub (crate) fn indices < C : cfg :: Config > (idx : usize) -> (Addr < C > , usize) { let addr = C :: unpack_addr (idx) ; (addr , addr . index ()) }
};
}
