// Generated macro for impl_48 (impl)
macro_rules! Depcrate_decoding_errorsimpl_48 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_48"}
// Dependencies: {}
impl fmt :: Display for ReadFrameHeaderError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: MagicNumberReadError (e) => write ! (f , "Error while reading magic number: {e}") , Self :: BadMagicNumber (e) => write ! (f , "Read wrong magic number: 0x{e:X}") , Self :: FrameDescriptorReadError (e) => { write ! (f , "Error while reading frame descriptor: {e}") } Self :: InvalidFrameDescriptor (e) => write ! (f , "{e:?}") , Self :: WindowDescriptorReadError (e) => { write ! (f , "Error while reading window descriptor: {e}") } Self :: DictionaryIdReadError (e) => write ! (f , "Error while reading dictionary id: {e}") , Self :: FrameContentSizeReadError (e) => { write ! (f , "Error while reading frame content size: {e}") } Self :: SkipFrame { magic_number , length , } => write ! (f , "SkippableFrame encountered with MagicNumber 0x{magic_number:X} and length {length} bytes") , } } }
};
}
