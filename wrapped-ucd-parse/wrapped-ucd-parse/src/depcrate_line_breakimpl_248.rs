// Generated macro for impl_248 (impl)
macro_rules! Depcrate_line_breakimpl_248 {
() => {
// Module: crate::line_break
// Provides: {"impl_248"}
// Dependencies: {}
impl std :: str :: FromStr for LineBreakTest { type Err = Error ; fn from_str (line : & str) -> Result < LineBreakTest , Error > { let (groups , comment) = parse_break_test (line) ? ; Ok (LineBreakTest { lines : groups , comment }) } }
};
}
