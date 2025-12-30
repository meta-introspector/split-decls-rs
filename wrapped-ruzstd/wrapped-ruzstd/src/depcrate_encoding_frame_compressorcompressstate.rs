// Generated macro for CompressState (struct)
macro_rules! Depcrate_encoding_frame_compressorCompressState {
() => {
// Module: crate::encoding::frame_compressor
// Provides: {"CompressState"}
// Dependencies: {}
pub (crate) struct CompressState < M : Matcher > { pub (crate) matcher : M , pub (crate) last_huff_table : Option < crate :: huff0 :: huff0_encoder :: HuffmanTable > , pub (crate) fse_tables : FseTables , }
};
}
