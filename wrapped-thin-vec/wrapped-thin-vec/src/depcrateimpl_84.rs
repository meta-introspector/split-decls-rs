// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < T > Drop for IntoIter < T > { # [inline] fn drop (& mut self) { # [cold] # [inline (never)] fn drop_non_singleton < T > (this : & mut IntoIter < T >) { unsafe { let mut vec = mem :: replace (& mut this . vec , ThinVec :: new ()) ; ptr :: drop_in_place (& mut vec [this . start ..]) ; vec . set_len_non_singleton (0) } } if ! self . vec . is_singleton () { drop_non_singleton (self) ; } } }
};
}
