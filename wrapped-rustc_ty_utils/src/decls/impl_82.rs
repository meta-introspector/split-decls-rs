macro_rules! deps {
    () => {
        NeedsDropTypes!();
        NeedsDropResult!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'tcx , F > NeedsDropTypes < 'tcx , F > { fn new (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , exhaustive : bool , adt_components : F ,) -> Self { let mut seen_tys = FxHashSet :: default () ; seen_tys . insert (ty) ; Self { tcx , typing_env , seen_tys , query_ty : ty , unchecked_tys : vec ! [(ty , 0)] , recursion_limit : tcx . recursion_limit () , adt_components , exhaustive , } } # [doc = " Called when `ty` is found to always require drop."] # [doc = " If the exhaustive flag is true, then `Ok(ty)` is returned like any other type."] # [doc = " Otherwise, `Err(AlwaysRequireDrop)` is returned, which will cause iteration to abort."] fn always_drop_component (& self , ty : Ty < 'tcx >) -> NeedsDropResult < Ty < 'tcx > > { if self . exhaustive { Ok (ty) } else { Err (AlwaysRequiresDrop) } } }
    };
}

impl_82!();