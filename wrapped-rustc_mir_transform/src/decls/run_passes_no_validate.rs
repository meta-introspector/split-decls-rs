macro_rules! deps {
    () => {
        Optimizations!();
        MirPass!();
    };
}

macro_rules! run_passes_no_validate {
    () => {
        deps!();
        # [doc = " Run the sequence of passes without validating the MIR after each pass. The MIR is still"] # [doc = " validated at the end."] pub (super) fn run_passes_no_validate < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , passes : & [& dyn MirPass < 'tcx >] , phase_change : Option < MirPhase > ,) { run_passes_inner (tcx , body , passes , phase_change , false , Optimizations :: Allowed) ; }
    };
}

run_passes_no_validate!();