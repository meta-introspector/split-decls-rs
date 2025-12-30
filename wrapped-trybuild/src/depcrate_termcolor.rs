// Generated macro for color (function)
macro_rules! Depcrate_termcolor {
() => {
// Module: crate::term
// Provides: {"color"}
// Dependencies: {}
pub (crate) fn color (color : Color) { lock () . set_color (ColorSpec :: new () . set_fg (Some (color))) ; }
};
}
