// Generated macro for Waiter (type)
macro_rules! Depcrate_query_jobWaiter {
() => {
// Module: crate::query::job
// Provides: {"Waiter"}
// Dependencies: {}
# [doc = " A resumable waiter of a query. The usize is the index into waiters in the query's latch"] type Waiter = (QueryJobId , usize) ;
};
}
