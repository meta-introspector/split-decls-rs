// Generated macro for impl_45 (impl)
macro_rules! Depcrate_decoding_errorsimpl_45 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "std")] impl StdError for FrameHeaderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { FrameHeaderError :: FrameDescriptorError (source) => Some (source) , _ => None , } } }
};
}
