// Generated macro for impl_66 (impl)
macro_rules! Depcrate_hypertreeimpl_66 {
() => {
// Module: crate::hypertree
// Provides: {"impl_66"}
// Dependencies: {}
impl < P : HypertreeParams > HypertreeSig < P > { pub (crate) const SIZE : usize = XmssSig :: < P > :: SIZE * P :: D :: USIZE ; pub (crate) fn write_to (& self , buf : & mut [u8]) { debug_assert ! (buf . len () == Self :: SIZE , "HT serialize length mismatch: {}, {}" , buf . len () , Self :: SIZE) ; buf . chunks_exact_mut (XmssSig :: < P > :: SIZE) . zip (self . 0 . iter ()) . for_each (| (buf , sig) | sig . write_to (buf)) ; } # [cfg (feature = "alloc")] pub (crate) fn to_vec (& self) -> alloc :: vec :: Vec < u8 > { let mut buf = alloc :: vec ! [0u8 ; Self :: SIZE] ; self . write_to (& mut buf) ; buf } }
};
}
