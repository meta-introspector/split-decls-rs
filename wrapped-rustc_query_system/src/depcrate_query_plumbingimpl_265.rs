// Generated macro for impl_265 (impl)
macro_rules! Depcrate_query_plumbingimpl_265 {
() => {
// Module: crate::query::plumbing
// Provides: {"impl_265"}
// Dependencies: {}
impl < I > QueryResult < I > { # [doc = " Unwraps the query job expecting that it has started."] fn expect_job (self) -> QueryJob < I > { match self { Self :: Started (job) => job , Self :: Poisoned => { panic ! ("job for query failed to start and was poisoned") } } } }
};
}
