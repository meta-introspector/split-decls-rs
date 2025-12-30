// Generated macro for encode_fse_table_modes (function)
macro_rules! Depcrate_encoding_blocks_compressedencode_fse_table_modes {
() => {
// Module: crate::encoding::blocks::compressed
// Provides: {"encode_fse_table_modes"}
// Dependencies: {}
fn encode_fse_table_modes (ll_mode : & FseTableMode < '_ > , ml_mode : & FseTableMode < '_ > , of_mode : & FseTableMode < '_ > ,) -> u8 { fn mode_to_bits (mode : & FseTableMode < '_ >) -> u8 { match mode { FseTableMode :: Predefined (_) => 0 , FseTableMode :: Encoded (_) => 2 , FseTableMode :: RepeateLast (_) => 3 , } } mode_to_bits (ll_mode) << 6 | mode_to_bits (of_mode) << 4 | mode_to_bits (ml_mode) << 2 }
};
}
