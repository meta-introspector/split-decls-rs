// Generated macro for impl_601 (impl)
macro_rules! Depcrate_ule_encodeimpl_601 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_601"}
// Dependencies: {}
unsafe impl < T > EncodeAsVarULE < ZeroSlice < T > > for & '_ [T] where T : AsULE + 'static , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { self . len () * core :: mem :: size_of :: < T :: ULE > () } fn encode_var_ule_write (& self , dst : & mut [u8]) { # [allow (non_snake_case)] let S = core :: mem :: size_of :: < T :: ULE > () ; debug_assert_eq ! (self . len () * S , dst . len ()) ; for (item , ref mut chunk) in self . iter () . zip (dst . chunks_mut (S)) { let ule = item . to_unaligned () ; chunk . copy_from_slice (ULE :: slice_as_bytes (core :: slice :: from_ref (& ule))) ; } } }
};
}
