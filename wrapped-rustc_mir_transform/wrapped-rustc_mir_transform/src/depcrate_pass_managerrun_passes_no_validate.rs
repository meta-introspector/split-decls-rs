// Generated macro for run_passes_no_validate (function)
macro_rules! Depcrate_pass_managerrun_passes_no_validate {
() => {
// Module: crate::pass_manager
// Provides: {"run_passes_no_validate"}
// Dependencies: {}
# [doc = " Run the sequence of passes without validating the MIR after each pass. The MIR is still"] # [doc = " validated at the end."] pub (super) fn run_passes_no_validate < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , passes : & [& dyn MirPass < 'tcx >] , phase_change : Option < MirPhase > ,) { run_passes_inner (tcx , body , passes , phase_change , false , Optimizations :: Allowed) ; }
};
}
