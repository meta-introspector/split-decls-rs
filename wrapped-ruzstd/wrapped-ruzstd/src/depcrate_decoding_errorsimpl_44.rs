// Generated macro for impl_44 (impl)
macro_rules! Depcrate_decoding_errorsimpl_44 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_44"}
// Dependencies: {}
impl fmt :: Display for FrameHeaderError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: WindowTooBig { got } => write ! (f , "window_size bigger than allowed maximum. Is: {}, Should be lower than: {}" , got , crate :: common :: MAX_WINDOW_SIZE) , Self :: WindowTooSmall { got } => write ! (f , "window_size smaller than allowed minimum. Is: {}, Should be greater than: {}" , got , crate :: common :: MIN_WINDOW_SIZE) , Self :: FrameDescriptorError (e) => write ! (f , "{e:?}") , Self :: DictIdTooSmall { got , expected } => write ! (f , "Not enough bytes in dict_id. Is: {got}, Should be: {expected}") , Self :: MismatchedFrameSize { got , expected } => write ! (f , "frame_content_size does not have the right length. Is: {got}, Should be: {expected}") , Self :: FrameSizeIsZero => write ! (f , "frame_content_size was zero") , Self :: InvalidFrameSize { got } => write ! (f , "Invalid frame_content_size. Is: {got}, Should be one of 1, 2, 4, 8 bytes") , } } }
};
}
