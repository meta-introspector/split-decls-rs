// Generated macro for impl_611 (impl)
macro_rules! Depcrate_updateimpl_611 {
() => {
// Module: crate::update
// Provides: {"impl_611"}
// Dependencies: {}
unsafe impl < T > Update for Arc < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_arc : Self) -> bool { let old_arc : & mut Arc < T > = unsafe { & mut * old_pointer } ; if Arc :: ptr_eq (old_arc , & new_arc) { return false ; } if let Some (inner) = Arc :: get_mut (old_arc) { match Arc :: try_unwrap (new_arc) { Ok (new_inner) => unsafe { T :: maybe_update (inner , new_inner) } , Err (new_arc) => { * old_arc = new_arc ; true } } } else { unsafe { * old_pointer = new_arc } ; true } } }
};
}
