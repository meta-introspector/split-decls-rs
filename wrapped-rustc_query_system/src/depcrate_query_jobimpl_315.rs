// Generated macro for impl_315 (impl)
macro_rules! Depcrate_query_jobimpl_315 {
() => {
// Module: crate::query::job
// Provides: {"impl_315"}
// Dependencies: {}
impl < I > QueryInfo < I > { pub (crate) fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryInfo < QueryStackFrameExtra > { QueryInfo { span : self . span , query : self . query . lift (qcx) } } }
};
}
