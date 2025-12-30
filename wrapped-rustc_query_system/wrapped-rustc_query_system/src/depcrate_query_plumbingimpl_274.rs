// Generated macro for impl_274 (impl)
macro_rules! Depcrate_query_plumbingimpl_274 {
() => {
// Module: crate::query::plumbing
// Provides: {"impl_274"}
// Dependencies: {}
impl < I > CycleError < I > { fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx) -> CycleError < QueryStackFrameExtra > { CycleError { usage : self . usage . as_ref () . map (| (span , frame) | (* span , frame . lift (qcx))) , cycle : self . cycle . iter () . map (| info | info . lift (qcx)) . collect () , } } }
};
}
