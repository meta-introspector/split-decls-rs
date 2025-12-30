// Generated macro for impl_609 (impl)
macro_rules! Depcrate_updateimpl_609 {
() => {
// Module: crate::update
// Provides: {"impl_609"}
// Dependencies: {}
unsafe impl < T , const N : usize > Update for [T ; N] where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { let old_pointer : * mut T = unsafe { std :: ptr :: addr_of_mut ! ((* old_pointer) [0]) } ; let mut changed = false ; for (new_element , i) in new_vec . into_iter () . zip (0 ..) { changed |= unsafe { T :: maybe_update (old_pointer . add (i) , new_element) } ; } changed } }
};
}
