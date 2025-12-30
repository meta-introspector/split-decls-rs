// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl < T > Clone for ThinVec < T > where T : Clone , { # [inline] fn clone (& self) -> ThinVec < T > { # [cold] # [inline (never)] fn clone_non_singleton < T : Clone > (this : & ThinVec < T >) -> ThinVec < T > { let len = this . len () ; let mut new_vec = ThinVec :: < T > :: with_capacity (len) ; let mut data_raw = new_vec . data_raw () ; for x in this . iter () { unsafe { ptr :: write (data_raw , x . clone ()) ; data_raw = data_raw . add (1) ; } } unsafe { new_vec . set_len (len) ; } new_vec } if self . is_singleton () { ThinVec :: new () } else { clone_non_singleton (self) } } }
};
}
