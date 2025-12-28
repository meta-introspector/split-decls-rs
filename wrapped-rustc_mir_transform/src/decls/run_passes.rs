macro_rules! deps {
    () => {
        Optimizations!();
        MirPass!();
    };
}

macro_rules! run_passes {
    () => {
        deps!();
        # [doc = " The optional `phase_change` is applied after executing all the passes, if present"] pub (super) fn run_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , passes : & [& dyn MirPass < 'tcx >] , phase_change : Option < MirPhase > , optimizations : Optimizations ,) { run_passes_inner (tcx , body , passes , phase_change , true , optimizations) ; }
    };
}

run_passes!()