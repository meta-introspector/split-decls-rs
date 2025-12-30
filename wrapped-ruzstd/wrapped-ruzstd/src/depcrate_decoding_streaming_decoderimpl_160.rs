// Generated macro for impl_160 (impl)
macro_rules! Depcrate_decoding_streaming_decoderimpl_160 {
() => {
// Module: crate::decoding::streaming_decoder
// Provides: {"impl_160"}
// Dependencies: {}
impl < READ : Read , DEC : BorrowMut < FrameDecoder > > StreamingDecoder < READ , DEC > { # [doc = " Gets a reference to the underlying reader."] pub fn get_ref (& self) -> & READ { & self . source } # [doc = " Gets a mutable reference to the underlying reader."] # [doc = ""] # [doc = " It is inadvisable to directly read from the underlying reader."] pub fn get_mut (& mut self) -> & mut READ { & mut self . source } # [doc = " Destructures this object into the inner reader."] pub fn into_inner (self) -> READ where READ : Sized , { self . source } # [doc = " Destructures this object into both the inner reader and [FrameDecoder]."] pub fn into_parts (self) -> (READ , DEC) where READ : Sized , { (self . source , self . decoder) } # [doc = " Destructures this object into the inner [FrameDecoder]."] pub fn into_frame_decoder (self) -> DEC { self . decoder } }
};
}
