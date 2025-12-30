// Generated macro for is_xml11_name_start_char (function)
macro_rules! Depcrateis_xml11_name_start_char {
() => {
// Module: crate
// Provides: {"is_xml11_name_start_char"}
// Dependencies: {}
# [doc = " Almost all characters can form a name. Citation from <https://www.w3.org/TR/xml11/#sec-xml11>:"] # [doc = ""] # [doc = " > The overall philosophy of names has changed since XML 1.0. Whereas XML 1.0"] # [doc = " > provided a rigid definition of names, wherein everything that was not permitted"] # [doc = " > was forbidden, XML 1.1 names are designed so that everything that is not"] # [doc = " > forbidden (for a specific reason) is permitted. Since Unicode will continue"] # [doc = " > to grow past version 4.0, further changes to XML can be avoided by allowing"] # [doc = " > almost any character, including those not yet assigned, in names."] # [doc = ""] # [doc = " <https://www.w3.org/TR/xml11/#NT-NameStartChar>"] fn is_xml11_name_start_char (ch : char) -> bool { match ch { ':' | 'A' ..= 'Z' | '_' | 'a' ..= 'z' | '\u{00C0}' ..= '\u{00D6}' | '\u{00D8}' ..= '\u{00F6}' | '\u{00F8}' ..= '\u{02FF}' | '\u{0370}' ..= '\u{037D}' | '\u{037F}' ..= '\u{1FFF}' | '\u{200C}' ..= '\u{200D}' | '\u{2070}' ..= '\u{218F}' | '\u{2C00}' ..= '\u{2FEF}' | '\u{3001}' ..= '\u{D7FF}' | '\u{F900}' ..= '\u{FDCF}' | '\u{FDF0}' ..= '\u{FFFD}' | '\u{10000}' ..= '\u{EFFFF}' => true , _ => false , } }
};
}
