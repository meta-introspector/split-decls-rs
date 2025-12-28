macro_rules! deps {
    () => {
        SymbolPath!();
    };
}

macro_rules! LegacySymbolMangler {
    () => {
        deps!();
        struct LegacySymbolMangler < 'tcx > { tcx : TyCtxt < 'tcx > , path : SymbolPath , keep_within_component : bool , }
    };
}

LegacySymbolMangler!();