// Generated macro for StreamMock (struct)
macro_rules! Depcrate_stream_mockStreamMock {
() => {
// Module: crate::stream_mock
// Provides: {"StreamMock"}
// Dependencies: {}
# [doc = " A mock stream implementing [`Stream`]"] # [doc = ""] # [doc = " See [`StreamMockBuilder`] for more information."] # [derive (Debug)] pub struct StreamMock < T : Unpin > { actions : VecDeque < Action < T > > , sleep : Option < Pin < Box < Sleep > > > , }
};
}
