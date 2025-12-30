// Generated macro for reserve_at_least (function)
macro_rules! Depcrate_filters_fsreserve_at_least {
() => {
// Module: crate::filters::fs
// Provides: {"reserve_at_least"}
// Dependencies: {}
fn reserve_at_least (buf : & mut BytesMut , cap : usize) { if buf . capacity () - buf . len () < cap { buf . reserve (cap) ; } }
};
}
