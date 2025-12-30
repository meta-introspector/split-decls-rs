// Generated macro for impl_22 (impl)
macro_rules! Depcrate_pointimpl_22 {
() => {
// Module: crate::point
// Provides: {"impl_22"}
// Dependencies: {}
impl < Size > Hash for EncodedPoint < Size > where Size : ModulusSize , { fn hash < H : Hasher > (& self , state : & mut H) { self . as_bytes () . hash (state) } }
};
}
