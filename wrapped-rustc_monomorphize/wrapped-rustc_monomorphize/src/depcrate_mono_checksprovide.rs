// Generated macro for provide (function)
macro_rules! Depcrate_mono_checksprovide {
() => {
// Module: crate::mono_checks
// Provides: {"provide"}
// Dependencies: {}
pub (super) fn provide (providers : & mut Providers) { * providers = Providers { check_mono_item , skip_move_check_fns : move_check :: skip_move_check_fns , .. * providers } }
};
}
