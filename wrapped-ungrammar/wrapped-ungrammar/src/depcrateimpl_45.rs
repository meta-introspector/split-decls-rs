// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl FromStr for Grammar { type Err = Error ; fn from_str (s : & str) -> Result < Self > { let tokens = lexer :: tokenize (s) ? ; parser :: parse (tokens) } }
};
}
