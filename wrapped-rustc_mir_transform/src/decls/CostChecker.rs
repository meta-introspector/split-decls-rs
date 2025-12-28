macro_rules! CostChecker {
    () => {
        struct CostChecker < 'b , 'tcx > { tcx : TyCtxt < 'tcx > , callee_body : & 'b Body < 'tcx > , calls : usize , statements : usize , landing_pads : usize , resumes : usize , }
    };
}

CostChecker!();