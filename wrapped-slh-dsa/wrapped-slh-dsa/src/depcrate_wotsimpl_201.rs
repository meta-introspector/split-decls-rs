// Generated macro for impl_201 (impl)
macro_rules! Depcrate_wotsimpl_201 {
() => {
// Module: crate::wots
// Provides: {"impl_201"}
// Dependencies: {}
impl < P : WotsParams > WotsSig < P > { pub (crate) const SIZE : usize = P :: N :: USIZE * P :: WotsSigLen :: USIZE ; pub (crate) fn write_to (& self , buf : & mut [u8]) { debug_assert ! (buf . len () == Self :: SIZE , "WOTS+ serialize length mismatch") ; buf . chunks_exact_mut (P :: N :: USIZE) . zip (self . 0 . iter ()) . for_each (| (buf , sig) | buf . copy_from_slice (sig . as_slice ())) ; } # [cfg (feature = "alloc")] # [cfg (test)] pub (crate) fn to_vec (& self) -> alloc :: vec :: Vec < u8 > { let mut vec = alloc :: vec ! [0u8 ; Self :: SIZE] ; self . write_to (& mut vec) ; vec } }
};
}
