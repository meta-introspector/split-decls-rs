// Generated macro for impl_287 (impl)
macro_rules! Depcrate_filters_bodyimpl_287 {
() => {
// Module: crate::filters::body
// Provides: {"impl_287"}
// Dependencies: {}
impl StdError for BodyDeserializeError { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (self . cause . as_ref ()) } }
};
}
