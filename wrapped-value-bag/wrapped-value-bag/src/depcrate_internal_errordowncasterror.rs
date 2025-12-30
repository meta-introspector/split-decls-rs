// Generated macro for DowncastError (trait)
macro_rules! Depcrate_internal_errorDowncastError {
() => {
// Module: crate::internal::error
// Provides: {"DowncastError"}
// Dependencies: {}
# [cfg (feature = "error")] pub (crate) trait DowncastError { fn as_any (& self) -> & dyn Any ; fn as_super (& self) -> & (dyn error :: Error + 'static) ; }
};
}
