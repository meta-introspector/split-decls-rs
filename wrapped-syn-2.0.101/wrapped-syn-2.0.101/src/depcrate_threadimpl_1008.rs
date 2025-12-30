// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_threadimpl_1008 {
() => {
// Module: crate::thread
// Provides: {"impl_1008"}
// Dependencies: {}
impl < T > ThreadBound < T > { pub (crate) fn new (value : T) -> Self { ThreadBound { value , thread_id : thread :: current () . id () , } } pub (crate) fn get (& self) -> Option < & T > { if thread :: current () . id () == self . thread_id { Some (& self . value) } else { None } } }
};
}
