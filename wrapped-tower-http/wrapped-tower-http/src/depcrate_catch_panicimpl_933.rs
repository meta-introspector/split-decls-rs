// Generated macro for impl_933 (impl)
macro_rules! Depcrate_catch_panicimpl_933 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_933"}
// Dependencies: {}
impl < F , B > ResponseForPanic for F where F : FnMut (Box < dyn Any + Send + 'static >) -> Response < B > + Clone , { type ResponseBody = B ; fn response_for_panic (& mut self , err : Box < dyn Any + Send + 'static > ,) -> Response < Self :: ResponseBody > { self (err) } }
};
}
