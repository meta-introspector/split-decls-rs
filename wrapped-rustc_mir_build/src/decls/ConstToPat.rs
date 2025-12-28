macro_rules! ConstToPat {
    () => {
        struct ConstToPat < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , span : Span , id : hir :: HirId , c : ty :: Const < 'tcx > , }
    };
}

ConstToPat!();