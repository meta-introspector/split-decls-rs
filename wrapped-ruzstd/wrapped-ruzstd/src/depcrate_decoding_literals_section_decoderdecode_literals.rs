// Generated macro for decode_literals (function)
macro_rules! Depcrate_decoding_literals_section_decoderdecode_literals {
() => {
// Module: crate::decoding::literals_section_decoder
// Provides: {"decode_literals"}
// Dependencies: {}
# [doc = " Decode and decompress the provided literals section into `target`, returning the number of bytes read."] pub fn decode_literals (section : & LiteralsSection , scratch : & mut HuffmanScratch , source : & [u8] , target : & mut Vec < u8 > ,) -> Result < u32 , DecompressLiteralsError > { match section . ls_type { LiteralsSectionType :: Raw => { target . extend (& source [0 .. section . regenerated_size as usize]) ; Ok (section . regenerated_size) } LiteralsSectionType :: RLE => { target . resize (target . len () + section . regenerated_size as usize , source [0]) ; Ok (1) } LiteralsSectionType :: Compressed | LiteralsSectionType :: Treeless => { let bytes_read = decompress_literals (section , scratch , source , target) ? ; Ok (bytes_read) } } }
};
}
