// Generated macro for impl_43 (impl)
macro_rules! Depcrate_inline_arrayimpl_43 {
() => {
// Module: crate::inline_array
// Provides: {"impl_43"}
// Dependencies: {}
impl < A , T > Debug for InlineArray < A , T > where A : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("Chunk") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}
