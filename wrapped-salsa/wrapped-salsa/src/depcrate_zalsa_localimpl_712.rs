// Generated macro for impl_712 (impl)
macro_rules! Depcrate_zalsa_localimpl_712 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_712"}
// Dependencies: {}
impl fmt :: Debug for QueryRevisionsExtraInner { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { struct FmtTrackedStructIds < 'a > (& 'a ThinVec < (Identity , Id) >) ; impl fmt :: Debug for FmtTrackedStructIds < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_list () ; if self . 0 . len () > 5 { f . entries (& self . 0 [.. 5]) ; f . finish_non_exhaustive () } else { f . entries (self . 0) ; f . finish () } } } let mut f = f . debug_struct ("QueryRevisionsExtraInner") ; f . field ("cycle_heads" , & self . cycle_heads) . field ("iteration" , & self . iteration) . field ("cycle_converged" , & self . cycle_converged) ; # [cfg (feature = "accumulator")] { f . field ("accumulated" , & self . accumulated) ; } f . field ("tracked_struct_ids" , & FmtTrackedStructIds (& self . tracked_struct_ids) ,) ; f . finish () } }
};
}
