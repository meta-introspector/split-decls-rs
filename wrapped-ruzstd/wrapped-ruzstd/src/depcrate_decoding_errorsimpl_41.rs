// Generated macro for impl_41 (impl)
macro_rules! Depcrate_decoding_errorsimpl_41 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_41"}
// Dependencies: {}
impl fmt :: Display for FrameDescriptorError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InvalidFrameContentSizeFlag { got } => write ! (f , "Invalid Frame_Content_Size_Flag; Is: {got}, Should be one of: 0, 1, 2, 3") , } } }
};
}
