// Generated macro for target (function)
macro_rules! Depcrate_cargotarget {
() => {
// Module: crate::cargo
// Provides: {"target"}
// Dependencies: {}
fn target () -> Vec < & 'static str > { if cfg ! (trybuild_no_target) { vec ! [] } else { vec ! ["--target" , TARGET] } }
};
}
