// Generated macro for impl_243 (impl)
macro_rules! Depcrate_async_helperimpl_243 {
() => {
// Module: crate::async_helper
// Provides: {"impl_243"}
// Dependencies: {}
impl TryWait for Pin < & mut AsyncWait > { # [inline] fn try_wait (& mut self , lock : & Lock) { let this = unsafe { ptr :: read (self) } ; let mut pinned_pager = unsafe { let pager_ref = std :: mem :: transmute :: < & mut Pager < 'static , Lock > , & mut Pager < Lock > > (& mut this . get_unchecked_mut () . pager ,) ; Pin :: new_unchecked (pager_ref) } ; lock . register_pager (& mut pinned_pager , Mode :: WaitExclusive , false) ; } }
};
}
