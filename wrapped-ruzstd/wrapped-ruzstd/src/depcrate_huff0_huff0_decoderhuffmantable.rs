// Generated macro for HuffmanTable (struct)
macro_rules! Depcrate_huff0_huff0_decoderHuffmanTable {
() => {
// Module: crate::huff0::huff0_decoder
// Provides: {"HuffmanTable"}
// Dependencies: {}
# [doc = " A Huffman decoding table contains a list of Huffman prefix codes and their associated values"] pub struct HuffmanTable { decode : Vec < Entry > , # [doc = " The weight of a symbol is the number of occurences in a table."] # [doc = " This value is used in constructing a binary tree referred to as"] # [doc = " a Huffman tree. Once this tree is constructed, it can be used to build the"] # [doc = " lookup table"] weights : Vec < u8 > , # [doc = " The maximum size in bits a prefix code in the encoded data can be."] # [doc = " This value is used so that the decoder knows how many bits"] # [doc = " to read from the bitstream before checking the table. This"] # [doc = " value must be 11 or lower."] pub max_num_bits : u8 , bits : Vec < u8 > , bit_ranks : Vec < u32 > , rank_indexes : Vec < usize > , # [doc = " In some cases, the list of weights is compressed using FSE compression."] fse_table : FSETable , }
};
}
