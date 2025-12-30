// Generated macro for BlockHeader (struct)
macro_rules! Depcrate_encoding_block_headerBlockHeader {
() => {
// Module: crate::encoding::block_header
// Provides: {"BlockHeader"}
// Dependencies: {}
# [derive (Debug)] pub struct BlockHeader { # [doc = " Signals if this block is the last one."] # [doc = " The frame will end after this block."] pub last_block : bool , # [doc = " Influences the meaning of `block_size`."] pub block_type : BlockType , # [doc = " - For `Raw` blocks, this is the size of the block's"] # [doc = "   content in bytes."] # [doc = " - For `RLE` blocks, there will be a single byte follwing"] # [doc = "   the header, repeated `block_size` times."] # [doc = " - For `Compressed` blocks, this is the length of"] # [doc = "   the compressed data."] # [doc = ""] # [doc = " **This value must not be greater than 21 bits in length.**"] pub block_size : u32 , }
};
}
