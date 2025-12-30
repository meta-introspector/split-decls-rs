// Generated macro for CycleError (struct)
macro_rules! Depcrate_query_plumbingCycleError {
() => {
// Module: crate::query::plumbing
// Provides: {"CycleError"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct CycleError < I = QueryStackFrameExtra > { # [doc = " The query and related span that uses the cycle."] pub usage : Option < (Span , QueryStackFrame < I >) > , pub cycle : Vec < QueryInfo < I > > , }
};
}
