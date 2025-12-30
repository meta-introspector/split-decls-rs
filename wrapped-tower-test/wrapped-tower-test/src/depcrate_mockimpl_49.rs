// Generated macro for impl_49 (impl)
macro_rules! Depcrate_mockimpl_49 {
() => {
// Module: crate::mock
// Provides: {"impl_49"}
// Dependencies: {}
impl < T , U > Drop for Handle < T , U > { fn drop (& mut self) { let mut state = match self . state . lock () { Ok (v) => v , Err (e) => { if :: std :: thread :: panicking () { return ; } panic ! ("{:?}" , e) ; } } ; state . is_closed = true ; for (_ , task) in state . tasks . drain () { task . wake () ; } } }
};
}
