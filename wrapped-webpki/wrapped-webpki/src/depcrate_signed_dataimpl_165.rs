// Generated macro for impl_165 (impl)
macro_rules! Depcrate_signed_dataimpl_165 {
() => {
// Module: crate::signed_data
// Provides: {"impl_165"}
// Dependencies: {}
impl core :: hash :: Hash for SignedData < '_ > { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { let Self { data , algorithm , signature , } = self ; data . as_slice_less_safe () . hash (state) ; algorithm . as_slice_less_safe () . hash (state) ; signature . as_slice_less_safe () . hash (state) ; } }
};
}
