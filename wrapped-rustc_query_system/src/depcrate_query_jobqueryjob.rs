// Generated macro for QueryJob (struct)
macro_rules! Depcrate_query_jobQueryJob {
() => {
// Module: crate::query::job
// Provides: {"QueryJob"}
// Dependencies: {}
# [doc = " Represents an active query job."] # [derive (Debug)] pub struct QueryJob < I > { pub id : QueryJobId , # [doc = " The span corresponding to the reason for which this query was required."] pub span : Span , # [doc = " The parent query job which created this job and is implicitly waiting on it."] pub parent : Option < QueryJobId > , # [doc = " The latch that is used to wait on this job."] latch : Option < QueryLatch < I > > , }
};
}
