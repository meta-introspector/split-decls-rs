// Generated macro for convert_primitive (macro)
macro_rules! Depcrate_internal_seqconvert_primitive {
() => {
// Module: crate::internal::seq
// Provides: {"convert_primitive"}
// Dependencies: {}
macro_rules ! convert_primitive (($ ($ t : ty ,) *) => { $ (impl <'v , const N : usize > From <&'v [$ t ; N] > for ValueBag <'v > { fn from (v : &'v [$ t ; N]) -> Self { ValueBag :: from_seq_slice (v) } } impl <'v , const N : usize > From < Option <&'v [$ t ; N] >> for ValueBag <'v > { fn from (v : Option <&'v [$ t ; N] >) -> Self { ValueBag :: from_option (v) } } impl <'a , 'v > From <&'v &'a [$ t] > for ValueBag <'v > { fn from (v : &'v &'a [$ t]) -> Self { ValueBag :: from_seq_slice (v) } } impl <'a , 'v > From < Option <&'v &'a [$ t] >> for ValueBag <'v > { fn from (v : Option <&'v &'a [$ t] >) -> Self { ValueBag :: from_option (v) } } # [cfg (feature = "alloc")] impl <'v > From <&'v Vec <$ t >> for ValueBag <'v > { fn from (v : &'v Vec <$ t >) -> Self { ValueBag :: from_seq_slice (v) } } # [cfg (feature = "alloc")] impl <'v > From < Option <&'v Vec <$ t >>> for ValueBag <'v > { fn from (v : Option <&'v Vec <$ t >>) -> Self { ValueBag :: from_option (v) } }) * }) ;
};
}
