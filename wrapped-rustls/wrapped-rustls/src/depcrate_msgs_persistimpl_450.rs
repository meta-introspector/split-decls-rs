// Generated macro for impl_450 (impl)
macro_rules! Depcrate_msgs_persistimpl_450 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_450"}
// Dependencies: {}
impl < T > Retrieved < T > { pub (crate) fn new (value : T , retrieved_at : UnixTime) -> Self { Self { value , retrieved_at , } } pub (crate) fn map < M > (& self , f : impl FnOnce (& T) -> Option < & M >) -> Option < Retrieved < & M > > { Some (Retrieved { value : f (& self . value) ? , retrieved_at : self . retrieved_at , }) } }
};
}
