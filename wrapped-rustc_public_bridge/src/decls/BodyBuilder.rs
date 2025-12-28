macro_rules! BodyBuilder {
    () => {
        # [doc = " Builds a monomorphic body for a given instance."] pub (crate) struct BodyBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , instance : ty :: Instance < 'tcx > , }
    };
}

BodyBuilder!();