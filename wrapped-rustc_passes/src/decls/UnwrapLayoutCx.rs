macro_rules! UnwrapLayoutCx {
    () => {
        struct UnwrapLayoutCx < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , }
    };
}

UnwrapLayoutCx!();