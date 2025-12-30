// Generated macro for impl_94 (impl)
macro_rules! Depcrate_parserimpl_94 {
() => {
// Module: crate::parser
// Provides: {"impl_94"}
// Dependencies: {}
impl < T : AsRef < str > > From < T > for SchemeType { fn from (s : T) -> Self { match s . as_ref () { "http" | "https" | "ws" | "wss" | "ftp" => Self :: SpecialNotFile , "file" => Self :: File , _ => Self :: NotSpecial , } } }
};
}
