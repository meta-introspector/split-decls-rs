macro_rules! deps {
    () => {
        HuffmanScratch!();
        FSEScratch!();
    };
}

macro_rules! Dictionary {
    () => {
        deps!();
        # [doc = " Zstandard includes support for \"raw content\" dictionaries, that store bytes optionally used"] # [doc = " during sequence execution."] # [doc = ""] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#dictionary-format>"] pub struct Dictionary { # [doc = " A 4 byte value used by decoders to check if they can use"] # [doc = " the correct dictionary. This value must not be zero."] pub id : u32 , # [doc = " A dictionary can contain an entropy table, either FSE or"] # [doc = " Huffman."] pub fse : FSEScratch , # [doc = " A dictionary can contain an entropy table, either FSE or"] # [doc = " Huffman."] pub huf : HuffmanScratch , # [doc = " The content of a dictionary acts as a \"past\" in front of data"] # [doc = " to compress or decompress,"] # [doc = " so it can be referenced in sequence commands."] # [doc = " As long as the amount of data decoded from this frame is less than or"] # [doc = " equal to Window_Size, sequence commands may specify offsets longer than"] # [doc = " the total length of decoded output so far to reference back to the"] # [doc = " dictionary, even parts of the dictionary with offsets larger than Window_Size."] # [doc = " After the total output has surpassed Window_Size however,"] # [doc = " this is no longer allowed and the dictionary is no longer accessible"] pub dict_content : Vec < u8 > , # [doc = " The 3 most recent offsets are stored so that they can be used"] # [doc = " during sequence execution, see"] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#repeat-offsets>"] # [doc = " for more."] pub offset_hist : [u32 ; 3] , }
    };
}

Dictionary!();