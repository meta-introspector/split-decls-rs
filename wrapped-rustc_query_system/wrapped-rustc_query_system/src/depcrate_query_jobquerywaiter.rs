// Generated macro for QueryWaiter (struct)
macro_rules! Depcrate_query_jobQueryWaiter {
() => {
// Module: crate::query::job
// Provides: {"QueryWaiter"}
// Dependencies: {}
# [derive (Debug)] struct QueryWaiter < I > { query : Option < QueryJobId > , condvar : Condvar , span : Span , cycle : Mutex < Option < CycleError < I > > > , }
};
}
