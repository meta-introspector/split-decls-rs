// Generated macro for impl_728 (impl)
macro_rules! Depcrate_visitimpl_728 {
() => {
// Module: crate::visit
// Provides: {"impl_728"}
// Dependencies: {}
impl < I : Interner > TypeVisitor < I > for HasErrorVisitor { type Result = ControlFlow < I :: ErrorGuaranteed > ; fn visit_error (& mut self , guar : < I as Interner > :: ErrorGuaranteed) -> Self :: Result { ControlFlow :: Break (guar) } }
};
}
