macro_rules! deps {
    () => {
        LetSource!();
    };
}

macro_rules! MatchVisitor {
    () => {
        deps!();
        struct MatchVisitor < 'p , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , thir : & 'p Thir < 'tcx > , lint_level : HirId , let_source : LetSource , pattern_arena : & 'p TypedArena < DeconstructedPat < 'p , 'tcx > > , dropless_arena : & 'p DroplessArena , # [doc = " Tracks if we encountered an error while checking this body. That the first function to"] # [doc = " report it stores it here. Some functions return `Result` to allow callers to short-circuit"] # [doc = " on error, but callers don't need to store it here again."] error : Result < () , ErrorGuaranteed > , }
    };
}

MatchVisitor!();