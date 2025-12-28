macro_rules! BlockType {
    () => {
        # [doc = " There are 4 different kinds of blocks, and the type of block influences the meaning of `Block_Size`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum BlockType { # [doc = " An uncompressed block."] Raw , # [doc = " A single byte, repeated `Block_Size` times (Run Length Encoding)."] # [allow (clippy :: upper_case_acronyms)] RLE , # [doc = " A Zstandard compressed block. `Block_Size` is the length of the compressed data."] Compressed , # [doc = " This is not a valid block, and this value should not be used."] # [doc = " If this value is present, it should be considered corrupted data."] Reserved , }
    };
}

BlockType!()