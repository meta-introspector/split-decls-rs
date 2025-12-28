macro_rules! deps {
    () => {
        LiteralsSectionType!();
    };
}

macro_rules! LiteralsSection {
    () => {
        deps!();
        # [doc = " A compressed block consists of two sections, a literals section, and a sequences section."] # [doc = ""] # [doc = " This is the first of those two sections. A literal is just any arbitrary data, and it is copied by the sequences section"] pub struct LiteralsSection { # [doc = " - If this block is of type [LiteralsSectionType::Raw], then the data is `regenerated_bytes`"] # [doc = "   bytes long, and it contains the raw literals data to be used during the second section,"] # [doc = "   the sequences section."] # [doc = " - If this block is of type [LiteralsSectionType::RLE],"] # [doc = "   then the literal consists of a single byte repeated `regenerated_size` times."] # [doc = " - For types [LiteralsSectionType::Compressed] or [LiteralsSectionType::Treeless],"] # [doc = "   then this is the size of the decompressed data."] pub regenerated_size : u32 , # [doc = " - For types [LiteralsSectionType::Raw] and [LiteralsSectionType::RLE], this value is not present."] # [doc = " - For types [LiteralsSectionType::Compressed] and [LiteralsSectionType::Treeless], this value will"] # [doc = "   be set to the size of the compressed data."] pub compressed_size : Option < u32 > , # [doc = " This value will be either 1 stream or 4 streams if the literal is of type"] # [doc = " [LiteralsSectionType::Compressed] or [LiteralsSectionType::Treeless], and it"] # [doc = " is not used for RLE or uncompressed literals."] pub num_streams : Option < u8 > , # [doc = " The type of the literal section."] pub ls_type : LiteralsSectionType , }
    };
}

LiteralsSection!()