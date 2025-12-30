// Generated macro for process_index (function)
macro_rules! Depcrateprocess_index {
() => {
// Module: crate
// Provides: {"process_index"}
// Dependencies: {}
fn process_index (enc : & 'static Encoding , codepoints : & Index) { make_xml (enc , codepoints . into_iter () . filter_map (| cp | { cp . map (| cp | char :: from_u32 (cp) . expect (& format ! ("`{}` is not a code point" , cp))) }) ,) }
};
}
