// Generated macro for unlikely (function)
macro_rules! Depcrate_hintunlikely {
() => {
// Module: crate::hint
// Provides: {"unlikely"}
// Dependencies: {}
# [doc = " Indicate that a given condition is likely to be false."] # [inline (always)] pub (crate) const fn unlikely (b : bool) -> bool { if b { cold_path () ; } b }
};
}
