macro_rules! MissingStabilityAnnotations {
    () => {
        struct MissingStabilityAnnotations < 'tcx > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'tcx EffectiveVisibilities , }
    };
}

MissingStabilityAnnotations!();