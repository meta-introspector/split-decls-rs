// Generated macro for impl_786 (impl)
macro_rules! Depcrate_uleimpl_786 {
() => {
// Module: crate::ule
// Provides: {"impl_786"}
// Dependencies: {}
impl UleError { # [doc = " Construct a parse error for the given type"] pub fn parse < T : ? Sized + 'static > () -> UleError { UleError :: ParseError { ty : any :: type_name :: < T > () , } } # [doc = " Construct an \"invalid length\" error for the given type and length"] pub fn length < T : ? Sized + 'static > (len : usize) -> UleError { UleError :: InvalidLength { ty : any :: type_name :: < T > () , len , } } }
};
}
