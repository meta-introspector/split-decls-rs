// Generated macro for impl_235 (impl)
macro_rules! Depcrate_lineimpl_235 {
() => {
// Module: crate::line
// Provides: {"impl_235"}
// Dependencies: {}
impl RuleBreakData < '_ > { fn get_linebreak_property_utf32_with_rule (& self , codepoint : u32 , strictness : LineBreakStrictness , word_option : LineBreakWordOption ,) -> u8 { let prop = self . property_table . get32 (codepoint) ; if word_option == LineBreakWordOption :: BreakAll || strictness == LineBreakStrictness :: Loose || strictness == LineBreakStrictness :: Normal { return match prop { CJ => ID , _ => prop , } ; } prop } # [inline] fn get_break_state_from_table (& self , left : u8 , right : u8) -> BreakState { let idx = (left as usize) * (self . property_count as usize) + (right as usize) ; self . break_state_table . get (idx) . unwrap_or (BreakState :: Keep) } # [inline] fn use_complex_breaking_utf32 (& self , codepoint : u32) -> bool { let line_break_property = self . get_linebreak_property_utf32_with_rule (codepoint , LineBreakStrictness :: Strict , LineBreakWordOption :: Normal ,) ; line_break_property == SA } }
};
}
