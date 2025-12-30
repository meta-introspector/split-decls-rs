// Generated macro for impl_46 (impl)
macro_rules! Depcrate_mockimpl_46 {
() => {
// Module: crate::mock
// Provides: {"impl_46"}
// Dependencies: {}
impl < T , U > Clone for Mock < T , U > { fn clone (& self) -> Self { let id = { let mut state = self . state . lock () . unwrap () ; let id = state . next_clone_id ; state . next_clone_id += 1 ; id } ; let tx = Mutex :: new (self . tx . lock () . unwrap () . clone ()) ; Mock { id , tx , state : self . state . clone () , can_send : false , } } }
};
}
