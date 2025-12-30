// Generated macro for macro_358 (macro)
macro_rules! Depcrate_load_peak_ewmamacro_358 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"macro_358"}
// Dependencies: {}
# [cfg (feature = "discover")] pin_project ! { # [doc = " Wraps a `D`-typed stream of discovered services with `PeakEwma`."] # [cfg_attr (docsrs , doc (cfg (feature = "discover")))] # [derive (Debug)] pub struct PeakEwmaDiscover < D , C = CompleteOnResponse > { # [pin] discover : D , decay_ns : f64 , default_rtt : Duration , completion : C , } }
};
}
