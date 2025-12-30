// Generated macro for Feature (struct)
macro_rules! Depcrate_featuresFeature {
() => {
// Module: crate::features
// Provides: {"Feature"}
// Dependencies: {}
# [derive (Debug , Clone)] # [cfg_attr (feature = "build-metrics" , derive (serde :: Serialize))] pub struct Feature { pub level : Status , pub since : Option < Version > , pub has_gate_test : bool , pub tracking_issue : Option < NonZeroU32 > , pub file : PathBuf , pub line : usize , pub description : Option < String > , }
};
}
