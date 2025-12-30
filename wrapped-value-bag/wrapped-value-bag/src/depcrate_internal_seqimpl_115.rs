// Generated macro for impl_115 (impl)
macro_rules! Depcrate_internal_seqimpl_115 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a , I : AsRef < [T] > + ? Sized + 'a , T > SeqSlice < 'a , I , T > { fn new_ref (v : & 'a I) -> & 'a SeqSlice < 'a , I , T > { unsafe { & * (v as * const I as * const SeqSlice < 'a , I , T >) } } fn as_ref < 'b > (& 'b self) -> & 'a [T] { let inner = unsafe { mem :: transmute :: < & 'b I , & 'a I > (& self . 1) } ; inner . as_ref () } }
};
}
