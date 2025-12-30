// Generated macro for impl_358 (impl)
macro_rules! Depcrate_metrics_labelsimpl_358 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_358"}
// Dependencies: {}
impl Serialize for QuicInvalidInitialPacketError { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . collect_str (self) } }
};
}
