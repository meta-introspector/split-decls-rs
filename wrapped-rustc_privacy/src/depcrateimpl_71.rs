// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'tcx > DefIdVisitor < 'tcx > for SearchInterfaceForPrivateItemsVisitor < 'tcx > { type Result = ControlFlow < () > ; fn skip_assoc_tys (& self) -> bool { self . skip_assoc_tys } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_def_id (& mut self , def_id : DefId , kind : & str , descr : & dyn fmt :: Display ,) -> Self :: Result { if self . check_def_id (def_id , kind , descr) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } }
};
}
