// Generated macro for encode_table (function)
macro_rules! Depcrate_encoding_blocks_compressedencode_table {
() => {
// Module: crate::encoding::blocks::compressed
// Provides: {"encode_table"}
// Dependencies: {}
fn encode_table (mode : & FseTableMode < '_ > , writer : & mut BitWriter < & mut Vec < u8 > >) { match mode { FseTableMode :: Predefined (_) => { } FseTableMode :: RepeateLast (_) => { } FseTableMode :: Encoded (table) => table . write_table (writer) , } }
};
}
