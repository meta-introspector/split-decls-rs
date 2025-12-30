// Generated macro for impl_213 (impl)
macro_rules! Depcrate_xmssimpl_213 {
() => {
// Module: crate::xmss
// Provides: {"impl_213"}
// Dependencies: {}
impl < P : XmssParams > XmssSig < P > { pub (crate) const SIZE : usize = WotsSig :: < P > :: SIZE + P :: HPrime :: USIZE * P :: N :: USIZE ; pub (crate) fn write_to (& self , buf : & mut [u8]) { debug_assert ! (buf . len () == Self :: SIZE , "Xmss serialize length mismatch") ; let (wots , auth) = buf . split_at_mut (WotsSig :: < P > :: SIZE) ; self . sig . write_to (wots) ; auth . chunks_exact_mut (P :: N :: USIZE) . zip (self . auth . iter ()) . for_each (| (buf , auth) | buf . copy_from_slice (auth . as_slice ())) ; } # [cfg (feature = "alloc")] # [cfg (test)] pub (crate) fn to_vec (& self) -> alloc :: vec :: Vec < u8 > { let mut buf = alloc :: vec ! [0u8 ; Self :: SIZE] ; self . write_to (& mut buf) ; buf } }
};
}
