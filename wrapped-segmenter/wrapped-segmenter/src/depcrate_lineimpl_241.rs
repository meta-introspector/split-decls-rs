// Generated macro for impl_241 (impl)
macro_rules! Depcrate_lineimpl_241 {
() => {
// Module: crate::line
// Provides: {"impl_241"}
// Dependencies: {}
impl < Y : LineBreakType > LineBreakIterator < '_ , '_ , Y > { fn advance_iter (& mut self) { self . current_pos_data = self . iter . next () ; } fn is_eof (& self) -> bool { self . current_pos_data . is_none () } # [inline] fn check_eof (& mut self) -> StringBoundaryPosType { if self . is_eof () { self . advance_iter () ; if self . is_eof () { if self . len == 0 { self . len = 1 ; StringBoundaryPosType :: Start } else { StringBoundaryPosType :: End } } else { StringBoundaryPosType :: Start } } else { StringBoundaryPosType :: Middle } } fn get_current_position (& self) -> Option < usize > { self . current_pos_data . map (| (pos , _) | pos) } fn get_current_codepoint (& self) -> Option < Y :: CharType > { self . current_pos_data . map (| (_ , codepoint) | codepoint) } fn get_linebreak_property (& self , codepoint : Y :: CharType) -> u8 { Y :: get_linebreak_property_with_rule (self , codepoint) } fn get_current_linebreak_property (& self) -> Option < u8 > { self . get_current_codepoint () . map (| c | self . get_linebreak_property (c)) } fn is_break_by_normal (& self , codepoint : Y :: CharType) -> bool { match codepoint . into () { 0x301C | 0x30A0 => self . options . ja_zh , _ => false , } } }
};
}
