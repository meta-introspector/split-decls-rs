// Generated macro for impl_253 (impl)
macro_rules! Depcrate_impls_livenessimpl_253 {
() => {
// Module: crate::impls::liveness
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'tcx > Analysis < 'tcx > for MaybeLiveLocals { type Domain = DenseBitSet < Local > ; type Direction = Backward ; const NAME : & 'static str = "liveness" ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (body . local_decls . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , _ : & mut Self :: Domain) { } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , statement : & mir :: Statement < 'tcx > , location : Location ,) { TransferFunction (state) . visit_statement (statement , location) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { TransferFunction (state) . visit_terminator (terminator , location) ; terminator . edges () } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , _block : mir :: BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { if let CallReturnPlaces :: Yield (resume_place) = return_places { YieldResumeEffect (state) . visit_place (& resume_place , PlaceContext :: MutatingUse (MutatingUseContext :: Yield) , Location :: START ,) } else { return_places . for_each (| place | { if let Some (local) = place . as_local () { state . kill (local) ; } }) ; } } }
};
}
