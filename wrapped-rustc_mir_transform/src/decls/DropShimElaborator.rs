macro_rules! deps {
    () => {
        MirPatch!();
    };
}

macro_rules! DropShimElaborator {
    () => {
        deps!();
        pub (super) struct DropShimElaborator < 'a , 'tcx > { pub body : & 'a Body < 'tcx > , pub patch : MirPatch < 'tcx > , pub tcx : TyCtxt < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , pub produce_async_drops : bool , }
    };
}

DropShimElaborator!()