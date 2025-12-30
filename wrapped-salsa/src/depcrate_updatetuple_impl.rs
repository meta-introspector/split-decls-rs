// Generated macro for tuple_impl (macro)
macro_rules! Depcrate_updatetuple_impl {
() => {
// Module: crate::update
// Provides: {"tuple_impl"}
// Dependencies: {}
macro_rules ! tuple_impl { ($ ($ t : ident) ,*; $ ($ u : ident) ,*) => { unsafe impl <$ ($ t) ,*> Update for ($ ($ t ,) *) where $ ($ t : Update ,) * { # [allow (non_snake_case)] unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { let ($ ($ t ,) *) = new_value ; let ($ ($ u ,) *) = unsafe { & mut * old_pointer } ; # [allow (unused_mut)] let mut changed = false ; $ (unsafe { changed |= Update :: maybe_update ($ u , $ t) ; }) * changed } } } }
};
}
