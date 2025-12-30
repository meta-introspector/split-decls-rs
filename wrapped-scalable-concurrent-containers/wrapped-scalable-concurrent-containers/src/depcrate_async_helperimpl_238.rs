// Generated macro for impl_238 (impl)
macro_rules! Depcrate_async_helperimpl_238 {
() => {
// Module: crate::async_helper
// Provides: {"impl_238"}
// Dependencies: {}
impl AsyncGuard { # [doc = " Returns `true` if the [`AsyncGuard`] contains a valid [`Guard`]."] # [inline] pub (crate) fn has_guard (& self) -> bool { unsafe { (* self . guard . get ()) . is_some () } } # [doc = " Returns or creates a new [`Guard`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that any references derived from the returned [`Guard`] do not"] # [doc = " outlive the underlying instance."] # [inline] pub (crate) fn guard (& self) -> & Guard { unsafe { (* self . guard . get ()) . get_or_insert_with (Guard :: new) } } # [doc = " Resets the [`AsyncGuard`] to its initial state."] # [inline] pub (crate) fn reset (& self) { unsafe { * self . guard . get () = None ; } } # [doc = " Loads the content of the [`AtomicShared`] without exposing the [`Guard`]."] # [inline] pub (crate) fn load < T > (& self , atomic_ptr : & AtomicShared < T > , mo : Ordering) -> Option < & T > { atomic_ptr . load (mo , self . guard ()) . as_ref () } # [doc = " Checks if the reference is valid."] # [inline] pub (crate) fn check_ref < T > (& self , atomic_ptr : & AtomicShared < T > , r : & T , mo : Ordering) -> bool { atomic_ptr . load (mo , self . guard ()) . as_ref () . is_some_and (| s | ptr :: eq (s , r)) } }
};
}
