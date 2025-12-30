// Generated macro for all_valid_chars (function)
macro_rules! Depcrateall_valid_chars {
() => {
// Module: crate
// Provides: {"all_valid_chars"}
// Dependencies: {}
# [doc = " A `char` in Rust is a Unicode Scalar Value"] # [doc = ""] # [doc = " See: http://www.unicode.org/glossary/#unicode_scalar_value"] fn all_valid_chars () -> impl Iterator < Item = char > { (0u32 ..= 0xD7FF) . chain (0xE000u32 ..= 0x10FFFF) . map (| u | { core :: convert :: TryFrom :: try_from (u) . expect ("The selected range should be infallible if the docs match impl") }) }
};
}
