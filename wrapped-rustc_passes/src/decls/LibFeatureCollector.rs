macro_rules! LibFeatureCollector {
    () => {
        struct LibFeatureCollector < 'tcx > { tcx : TyCtxt < 'tcx > , lib_features : LibFeatures , }
    };
}

LibFeatureCollector!();