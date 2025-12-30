// Generated macro for impl_47 (impl)
macro_rules! Depcrate_dataimpl_47 {
() => {
// Module: crate::data
// Provides: {"impl_47"}
// Dependencies: {}
impl Value for () { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { stream . tag (Some (& tags :: RUST_UNIT) , None , None) } }
};
}
