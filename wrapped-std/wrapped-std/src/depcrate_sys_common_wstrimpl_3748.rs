// Generated macro for impl_3748 (impl)
macro_rules! Depcrate_sys_common_wstrimpl_3748 {
() => {
// Module: crate::sys_common::wstr
// Provides: {"impl_3748"}
// Dependencies: {}
impl WStrUnits < '_ > { # [doc = " Creates the iterator. Returns `None` if `lpwstr` is null."] # [doc = ""] # [doc = " SAFETY: `lpwstr` must point to a null-terminated wide string that lives"] # [doc = " at least as long as the lifetime of this struct."] pub unsafe fn new (lpwstr : * const u16) -> Option < Self > { Some (Self { lpwstr : NonNull :: new (lpwstr as _) ? , lifetime : PhantomData }) } pub fn peek (& self) -> Option < NonZero < u16 > > { unsafe { NonZero :: new (* self . lpwstr . as_ptr ()) } } # [doc = " Advance the iterator while `predicate` returns true."] # [doc = " Returns the number of items it advanced by."] pub fn advance_while < P : FnMut (NonZero < u16 >) -> bool > (& mut self , mut predicate : P) -> usize { let mut counter = 0 ; while let Some (w) = self . peek () { if ! predicate (w) { break ; } counter += 1 ; self . next () ; } counter } }
};
}
