// Generated macro for bold_color (function)
macro_rules! Depcrate_termbold_color {
() => {
// Module: crate::term
// Provides: {"bold_color"}
// Dependencies: {}
pub (crate) fn bold_color (color : Color) { lock () . set_color (ColorSpec :: new () . set_bold (true) . set_fg (Some (color))) ; }
};
}
