macro_rules! deps {
    () => {
        BlockType!();
    };
}

macro_rules! BlockHeader {
    () => {
        deps!();
        # [doc = " A representation of a single block header. As well as containing a frame header,"] # [doc = " each Zstandard frame contains one or more blocks."] pub struct BlockHeader { # [doc = " Whether this block is the last block in the frame."] # [doc = " It may be followed by an optional `Content_Checksum` if it is."] pub last_block : bool , pub block_type : BlockType , # [doc = " The size of the decompressed data. If the block type"] # [doc = " is [BlockType::Reserved] or [BlockType::Compressed],"] # [doc = " this value is set to zero and should not be referenced."] pub decompressed_size : u32 , # [doc = " The size of the block. If the block is [BlockType::RLE],"] # [doc = " this value will be 1."] pub content_size : u32 , }
    };
}

BlockHeader!();