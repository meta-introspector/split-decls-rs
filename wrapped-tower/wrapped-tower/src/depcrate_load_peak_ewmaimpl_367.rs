// Generated macro for impl_367 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_367 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_367"}
// Dependencies: {}
# [cfg (feature = "discover")] impl < D , C > PeakEwmaDiscover < D , C > { # [doc = " Wraps a `D`-typed [`Discover`] so that services have a [`PeakEwma`] load metric."] # [doc = ""] # [doc = " The provided `default_rtt` is used as the default RTT estimate for newly"] # [doc = " added services."] # [doc = ""] # [doc = " They `decay` value determines over what time period a RTT estimate should"] # [doc = " decay."] pub fn new < Request > (discover : D , default_rtt : Duration , decay : Duration , completion : C) -> Self where D : Discover , D :: Service : Service < Request > , C : TrackCompletion < Handle , < D :: Service as Service < Request > > :: Response > , { PeakEwmaDiscover { discover , decay_ns : nanos (decay) , default_rtt , completion , } } }
};
}
