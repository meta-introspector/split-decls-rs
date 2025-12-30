// Generated macro for impl_43 (impl)
macro_rules! Depcrate_stream_mockimpl_43 {
() => {
// Module: crate::stream_mock
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : Unpin > StreamMock < T > { fn next_action (& mut self) -> Option < Action < T > > { self . actions . pop_front () } }
};
}
