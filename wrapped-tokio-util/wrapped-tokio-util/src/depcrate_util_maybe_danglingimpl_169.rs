// Generated macro for impl_169 (impl)
macro_rules! Depcrate_util_maybe_danglingimpl_169 {
() => {
// Module: crate::util::maybe_dangling
// Provides: {"impl_169"}
// Dependencies: {}
impl < F : Future > Future for MaybeDangling < F > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let fut = unsafe { self . map_unchecked_mut (| this | this . 0 . assume_init_mut ()) } ; fut . poll (cx) } }
};
}
