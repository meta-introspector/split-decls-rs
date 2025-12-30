// Generated macro for impl_85 (impl)
macro_rules! Depcrate_log_supportimpl_85 {
() => {
// Module: crate::log_support
// Provides: {"impl_85"}
// Dependencies: {}
impl < S : SerializeMap > SerdeMapVisitorStrippingLog < S > { fn new (serializer : S) -> SerdeMapVisitorStrippingLog < S > { SerdeMapVisitorStrippingLog (SerdeMapVisitor :: new (serializer)) } fn ignore (field : & Field) -> bool { field . name () . starts_with ("log.") } fn finish (self) -> Result < S :: Ok , S :: Error > { self . 0 . finish () } }
};
}
