// Generated macro for MockHandle (struct)
macro_rules! Depcrate_subscriberMockHandle {
() => {
// Module: crate::subscriber
// Provides: {"MockHandle"}
// Dependencies: {}
# [doc = " A handle which is used to invoke validation of expectations."] # [doc = ""] # [doc = " The handle is currently only used to assert that all the expected"] # [doc = " events and spans were seen."] # [doc = ""] # [doc = " For additional information and examples, see the [`subscriber`]"] # [doc = " module documentation."] # [doc = ""] # [doc = " [`subscriber`]: mod@crate::subscriber"] # [derive (Debug)] pub struct MockHandle (Arc < Mutex < VecDeque < Expect > > > , String) ;
};
}
