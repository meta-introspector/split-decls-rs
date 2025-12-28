macro_rules! LiteralsSectionType {
    () => {
        # [doc = " The way which a literal section is encoded."] pub enum LiteralsSectionType { # [doc = " Literals are stored uncompressed."] Raw , # [doc = " Literals consist of a single byte value repeated [LiteralsSection::regenerated_size] times."] # [allow (clippy :: upper_case_acronyms)] RLE , # [doc = " This is a standard Huffman-compressed block, starting with a Huffman tree description."] # [doc = " In this mode, there are at least *2* different literals represented in the Huffman tree"] # [doc = " description."] Compressed , # [doc = " This is a Huffman-compressed block,"] # [doc = " using the Huffman tree from the previous [LiteralsSectionType::Compressed] block"] # [doc = " in the sequence. If this mode is triggered without any previous Huffman-tables in the"] # [doc = " frame (or dictionary), it should be treated as data corruption."] Treeless , }
    };
}

LiteralsSectionType!();