// Generated macro for wtf8_meaningful (function)
macro_rules! Depcrate_fmtwtf8_meaningful {
() => {
// Module: crate::fmt
// Provides: {"wtf8_meaningful"}
// Dependencies: {}
# [inline] fn wtf8_meaningful (m : Meaning) -> bool { match m { Meaning :: Whole (_) | Meaning :: LeadSurrogate (_) | Meaning :: TrailSurrogate (_) => true , _ => false , } }
};
}
