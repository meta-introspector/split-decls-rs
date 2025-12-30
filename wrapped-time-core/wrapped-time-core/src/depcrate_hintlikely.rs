// Generated macro for likely (function)
macro_rules! Depcrate_hintlikely {
() => {
// Module: crate::hint
// Provides: {"likely"}
// Dependencies: {}
# [doc = " Indicate that a given condition is likely to be true."] # [inline (always)] pub (crate) const fn likely (b : bool) -> bool { if ! b { cold_path () ; } b }
};
}
