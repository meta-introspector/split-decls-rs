// Generated macro for is_valid_literal_block_scalar (function)
macro_rules! Depcrate_char_traitsis_valid_literal_block_scalar {
() => {
// Module: crate::char_traits
// Provides: {"is_valid_literal_block_scalar"}
// Dependencies: {}
# [doc = " Check if the string can be expressed a valid literal block scalar."] # [doc = " The YAML spec supports all of the following in block literals except `#xFEFF`:"] # [doc = " ```no_compile"] # [doc = "     #x9 | #xA | [#x20-#x7E]                /* 8 bit */"] # [doc = "   | #x85 | [#xA0-#xD7FF] | [#xE000-#xFFFD] /* 16 bit */"] # [doc = "   | [#x10000-#x10FFFF]                     /* 32 bit */"] # [doc = " ```"] # [inline] pub (crate) fn is_valid_literal_block_scalar (string : & str) -> bool { string . chars () . all (| character : char | matches ! (character , '\t' | '\n' | '\x20' ..='\x7e' | '\u{0085}' | '\u{00a0}' ..='\u{d7fff}')) }
};
}
