// Generated macro for TestEvent (enum)
macro_rules! Depcrate_eventTestEvent {
() => {
// Module: crate::event
// Provides: {"TestEvent"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum TestEvent { TeFiltered (usize , Option < u64 >) , TeWait (TestDesc) , TeResult (CompletedTest) , TeTimeout (TestDesc) , TeFilteredOut (usize) , }
};
}
