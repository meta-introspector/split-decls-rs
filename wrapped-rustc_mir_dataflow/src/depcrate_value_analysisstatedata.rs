// Generated macro for StateData (struct)
macro_rules! Depcrate_value_analysisStateData {
() => {
// Module: crate::value_analysis
// Provides: {"StateData"}
// Dependencies: {}
# [doc = " See [`State`]."] # [derive (PartialEq , Eq , Debug)] pub struct StateData < V > { bottom : V , # [doc = " This map only contains values that are not `⊥`."] map : FxHashMap < ValueIndex , V > , }
};
}
