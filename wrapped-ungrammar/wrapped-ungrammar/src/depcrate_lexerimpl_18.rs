// Generated macro for impl_18 (impl)
macro_rules! Depcrate_lexerimpl_18 {
() => {
// Module: crate::lexer
// Provides: {"impl_18"}
// Dependencies: {}
impl Location { fn advance (& mut self , text : & str) { match text . rfind ('\n') { Some (idx) => { self . line += text . chars () . filter (| & it | it == '\n') . count () ; self . column = text [idx + 1 ..] . chars () . count () ; } None => self . column += text . chars () . count () , } } }
};
}
