// Generated macro for impl_88 (impl)
macro_rules! Depcrate_decoding_errorsimpl_88 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_88"}
// Dependencies: {}
impl From < BlockHeaderReadError > for FrameDecoderError { fn from (val : BlockHeaderReadError) -> Self { Self :: FailedToReadBlockHeader (val) } }
};
}
