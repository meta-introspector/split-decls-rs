// Generated macro for impl_658 (impl)
macro_rules! Depcrate_ule_optionimpl_658 {
() => {
// Module: crate::ule::option
// Provides: {"impl_658"}
// Dependencies: {}
unsafe impl < T , U > EncodeAsVarULE < OptionVarULE < U > > for Option < T > where T : EncodeAsVarULE < U > , U : VarULE + ? Sized , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { if let Some (ref inner) = * self { 1 + inner . encode_var_ule_len () } else { 1 } } # [expect (clippy :: indexing_slicing)] fn encode_var_ule_write (& self , dst : & mut [u8]) { if let Some (ref inner) = * self { debug_assert ! (! dst . is_empty () , "OptionVarULE must have at least one byte when Some") ; dst [0] = 1 ; inner . encode_var_ule_write (& mut dst [1 ..]) ; } else { debug_assert ! (dst . len () == 1 , "OptionVarULE must have exactly one byte when None") ; dst [0] = 0 ; } } }
};
}
