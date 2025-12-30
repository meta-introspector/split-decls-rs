// Generated macro for bold (function)
macro_rules! Depcrate_termbold {
() => {
// Module: crate::term
// Provides: {"bold"}
// Dependencies: {}
pub (crate) fn bold () { lock () . set_color (ColorSpec :: new () . set_bold (true)) ; }
};
}
