// Generated macro for is_literal_xml11_char (function)
macro_rules! Depcrateis_literal_xml11_char {
() => {
// Module: crate
// Provides: {"is_literal_xml11_char"}
// Dependencies: {}
# [doc = " > XML 1.1 allows the use of character references to the control characters"] # [doc = " > #x1 through #x1F, most of which are forbidden in XML 1.0. For reasons of"] # [doc = " > robustness, however, these characters still cannot be used directly in"] # [doc = " > documents. In order to improve the robustness of character encoding detection,"] # [doc = " > the additional control characters #x7F through #x9F, which were freely allowed"] # [doc = " > in XML 1.0 documents, now must also appear only as character references."] # [doc = " > (Whitespace characters are of course exempt.)"] # [doc = ""] # [doc = " https://www.w3.org/TR/xml11/#sec-xml11"] fn is_literal_xml11_char (ch : char) -> bool { match ch { '\u{0001}' ..= '\u{D7FF}' => match ch { '\u{0001}' ..= '\u{0008}' => false , '\u{000B}' ..= '\u{000C}' => false , '\u{000E}' ..= '\u{001F}' => false , '\u{007F}' ..= '\u{0084}' => false , '\u{0086}' ..= '\u{009F}' => false , _ => true , } , '\u{E000}' ..= '\u{FFFD}' => true , '\u{10000}' ..= '\u{10FFFF}' => true , _ => false , } }
};
}
