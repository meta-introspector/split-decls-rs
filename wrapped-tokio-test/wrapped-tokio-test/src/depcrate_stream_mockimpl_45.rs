// Generated macro for impl_45 (impl)
macro_rules! Depcrate_stream_mockimpl_45 {
() => {
// Module: crate::stream_mock
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : Unpin > Drop for StreamMock < T > { fn drop (& mut self) { if std :: thread :: panicking () { return ; } let undropped_count = self . actions . iter () . filter (| action | match action { Action :: Next (_) => true , Action :: Wait (_) => false , }) . count () ; assert ! (undropped_count == 0 , "StreamMock was dropped before all actions were consumed, {undropped_count} actions were not consumed") ; } }
};
}
