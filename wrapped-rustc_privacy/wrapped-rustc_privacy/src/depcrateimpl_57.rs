// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'tcx > DefIdVisitor < 'tcx > for ReachEverythingInTheInterfaceVisitor < '_ , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . ev . tcx } fn visit_def_id (& mut self , def_id : DefId , _kind : & str , _descr : & dyn fmt :: Display) { if let Some (def_id) = def_id . as_local () { let max_vis = (self . level != Level :: ReachableThroughImplTrait) . then (| | self . ev . tcx . local_visibility (def_id)) ; self . ev . update_eff_vis (def_id , self . effective_vis , max_vis , self . level) ; } } }
};
}
