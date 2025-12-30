// Generated macro for maybe_update_vec (macro)
macro_rules! Depcrate_updatemaybe_update_vec {
() => {
// Module: crate::update
// Provides: {"maybe_update_vec"}
// Dependencies: {}
macro_rules ! maybe_update_vec { ($ old_pointer : expr , $ new_vec : expr , $ elem_ty : ty) => { { let old_pointer = $ old_pointer ; let new_vec = $ new_vec ; let old_vec : & mut Self = unsafe { & mut * old_pointer } ; if old_vec . len () != new_vec . len () { old_vec . clear () ; old_vec . extend (new_vec) ; return true ; } let mut changed = false ; for (old_element , new_element) in old_vec . iter_mut () . zip (new_vec) { changed |= unsafe { <$ elem_ty >:: maybe_update (old_element , new_element) } ; } changed } } ; }
};
}
