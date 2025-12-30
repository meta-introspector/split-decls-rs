// Generated macro for impl_518 (impl)
macro_rules! Depcrate_reachableimpl_518 {
() => {
// Module: crate::reachable
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'tcx > DefIdVisitor < 'tcx > for ReachableContext < 'tcx > { type Result = () ; fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_def_id (& mut self , def_id : DefId , _kind : & str , _descr : & dyn std :: fmt :: Display ,) -> Self :: Result { self . propagate_item (Res :: Def (self . tcx . def_kind (def_id) , def_id)) } }
};
}
