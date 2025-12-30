// Generated macro for impl_210 (impl)
macro_rules! Depcrate_impls_borrowed_localsimpl_210 {
() => {
// Module: crate::impls::borrowed_locals
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'tcx > Analysis < 'tcx > for MaybeBorrowedLocals { type Domain = DenseBitSet < Local > ; const NAME : & 'static str = "maybe_borrowed_locals" ; fn bottom_value (& self , body : & Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (body . local_decls () . len ()) } fn initialize_start_block (& self , _ : & Body < 'tcx > , _ : & mut Self :: Domain) { } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , statement : & Statement < 'tcx > , location : Location ,) { Self :: transfer_function (state) . visit_statement (statement , location) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { Self :: transfer_function (state) . visit_terminator (terminator , location) ; terminator . edges () } }
};
}
