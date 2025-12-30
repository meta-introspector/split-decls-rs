// Generated macro for run_passes (function)
macro_rules! Depcrate_pass_managerrun_passes {
() => {
// Module: crate::pass_manager
// Provides: {"run_passes"}
// Dependencies: {}
# [doc = " The optional `phase_change` is applied after executing all the passes, if present"] pub (super) fn run_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , passes : & [& dyn MirPass < 'tcx >] , phase_change : Option < MirPhase > , optimizations : Optimizations ,) { run_passes_inner (tcx , body , passes , phase_change , true , optimizations) ; }
};
}
