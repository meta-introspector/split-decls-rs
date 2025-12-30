// Generated macro for from_raw (function)
macro_rules! Depcrate_util_atomic_cellfrom_raw {
() => {
// Module: crate::util::atomic_cell
// Provides: {"from_raw"}
// Dependencies: {}
fn from_raw < T > (val : * mut T) -> Option < Box < T > > { if val . is_null () { None } else { Some (unsafe { Box :: from_raw (val) }) } }
};
}
