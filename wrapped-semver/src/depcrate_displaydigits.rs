// Generated macro for digits (function)
macro_rules! Depcrate_displaydigits {
() => {
// Module: crate::display
// Provides: {"digits"}
// Dependencies: {}
fn digits (val : u64) -> usize { if val < 10 { 1 } else { 1 + digits (val / 10) } }
};
}
