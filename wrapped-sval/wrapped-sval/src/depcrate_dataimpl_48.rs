// Generated macro for impl_48 (impl)
macro_rules! Depcrate_dataimpl_48 {
() => {
// Module: crate::data
// Provides: {"impl_48"}
// Dependencies: {}
impl Value for bool { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { stream . bool (* self) } fn tag (& self) -> Option < Tag > { None } fn to_bool (& self) -> Option < bool > { Some (* self) } }
};
}
