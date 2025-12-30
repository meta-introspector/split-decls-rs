// Generated macro for impl_693 (impl)
macro_rules! Depcrate_limit_bodyimpl_693 {
() => {
// Module: crate::limit::body
// Provides: {"impl_693"}
// Dependencies: {}
impl < B > ResponseBody < B > { fn payload_too_large () -> Self { Self { inner : ResponseBodyInner :: PayloadTooLarge { body : Full :: from (BODY) , } , } } pub (crate) fn new (body : B) -> Self { Self { inner : ResponseBodyInner :: Body { body } , } } }
};
}
