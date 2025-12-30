// Generated macro for QueryInfo (struct)
macro_rules! Depcrate_query_jobQueryInfo {
() => {
// Module: crate::query::job
// Provides: {"QueryInfo"}
// Dependencies: {}
# [doc = " Represents a span and a query key."] # [derive (Clone , Debug)] pub struct QueryInfo < I > { # [doc = " The span corresponding to the reason for which this query was required."] pub span : Span , pub query : QueryStackFrame < I > , }
};
}
