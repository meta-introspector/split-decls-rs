macro_rules! QueryCtxt {
    () => {
        # [derive (Copy , Clone)] pub struct QueryCtxt < 'tcx > { pub tcx : TyCtxt < 'tcx > , }
    };
}

QueryCtxt!();