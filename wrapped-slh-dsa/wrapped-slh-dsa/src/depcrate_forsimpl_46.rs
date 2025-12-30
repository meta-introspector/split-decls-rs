// Generated macro for impl_46 (impl)
macro_rules! Depcrate_forsimpl_46 {
() => {
// Module: crate::fors
// Provides: {"impl_46"}
// Dependencies: {}
impl < P : ForsParams > ForsSignature < P > { pub (crate) const SIZE : usize = P :: K :: USIZE * (P :: A :: USIZE + 1) * P :: N :: USIZE ; pub (crate) fn write_to (& self , slice : & mut [u8]) { debug_assert ! (slice . len () == Self :: SIZE , "Writing FORS sig to slice of incorrect length") ; slice . chunks_exact_mut (ForsMTSig :: < P > :: SIZE) . enumerate () . for_each (| (i , c) | self . 0 [i] . write_to (c)) ; } # [cfg (feature = "alloc")] pub (crate) fn to_vec (& self) -> alloc :: vec :: Vec < u8 > { let mut v = alloc :: vec ! [0u8 ; Self :: SIZE] ; self . write_to (& mut v) ; v } }
};
}
