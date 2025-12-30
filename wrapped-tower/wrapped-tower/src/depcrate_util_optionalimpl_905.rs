// Generated macro for impl_905 (impl)
macro_rules! Depcrate_util_optionalimpl_905 {
() => {
// Module: crate::util::optional
// Provides: {"impl_905"}
// Dependencies: {}
impl < T > Optional < T > { # [doc = " Create a new [`Optional`]."] pub const fn new < Request > (inner : Option < T >) -> Optional < T > where T : Service < Request > , T :: Error : Into < crate :: BoxError > , { Optional { inner } } }
};
}
