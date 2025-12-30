// Generated macro for impl_40 (impl)
macro_rules! Depcrate_stream_mockimpl_40 {
() => {
// Module: crate::stream_mock
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : Unpin > StreamMockBuilder < T > { # [doc = " Create a new empty [`StreamMockBuilder`]"] pub fn new () -> Self { StreamMockBuilder :: default () } # [doc = " Queue an item to be returned by the stream"] pub fn next (mut self , value : T) -> Self { self . actions . push_back (Action :: Next (value)) ; self } # [doc = " Queue the stream to wait for a duration"] pub fn wait (mut self , duration : Duration) -> Self { self . actions . push_back (Action :: Wait (duration)) ; self } # [doc = " Build the [`StreamMock`]"] pub fn build (self) -> StreamMock < T > { StreamMock { actions : self . actions , sleep : None , } } }
};
}
