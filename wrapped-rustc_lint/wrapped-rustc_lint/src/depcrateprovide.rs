// Generated macro for provide (function)
macro_rules! Depcrateprovide {
() => {
// Module: crate
// Provides: {"provide"}
// Dependencies: {}
pub fn provide (providers : & mut Providers) { levels :: provide (providers) ; expect :: provide (providers) ; foreign_modules :: provide (providers) ; * providers = Providers { lint_mod , .. * providers } ; }
};
}
