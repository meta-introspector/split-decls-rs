// Generated macro for impl_610 (impl)
macro_rules! Depcrate_updateimpl_610 {
() => {
// Module: crate::update
// Provides: {"impl_610"}
// Dependencies: {}
unsafe impl < T > Update for Box < [T] > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_box : Self) -> bool { let old_box : & mut Box < [T] > = unsafe { & mut * old_pointer } ; if old_box . len () == new_box . len () { let mut changed = false ; for (old_element , new_element) in old_box . iter_mut () . zip (new_box) { changed |= unsafe { T :: maybe_update (old_element , new_element) } ; } changed } else { * old_box = new_box ; true } } }
};
}
