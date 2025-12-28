macro_rules! Checker {
    () => {
        struct Checker < 'tcx > { tcx : TyCtxt < 'tcx > , }
    };
}

Checker!()