// Generated macro for impl_47 (impl)
macro_rules! Depcrate_mockimpl_47 {
() => {
// Module: crate::mock
// Provides: {"impl_47"}
// Dependencies: {}
impl < T , U > Drop for Mock < T , U > { fn drop (& mut self) { let mut state = match self . state . lock () { Ok (v) => v , Err (e) => { if :: std :: thread :: panicking () { return ; } panic ! ("{:?}" , e) ; } } ; state . tasks . remove (& self . id) ; } }
};
}
