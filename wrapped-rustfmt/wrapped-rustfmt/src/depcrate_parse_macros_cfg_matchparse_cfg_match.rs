// Generated macro for parse_cfg_match (function)
macro_rules! Depcrate_parse_macros_cfg_matchparse_cfg_match {
() => {
// Module: crate::parse::macros::cfg_match
// Provides: {"parse_cfg_match"}
// Dependencies: {}
pub (crate) fn parse_cfg_match < 'a > (psess : & 'a ParseSess , mac : & 'a ast :: MacCall ,) -> Result < Vec < ast :: Item > , & 'static str > { match catch_unwind (AssertUnwindSafe (| | parse_cfg_match_inner (psess , mac))) { Ok (Ok (items)) => Ok (items) , Ok (err @ Err (_)) => err , Err (..) => Err ("failed to parse cfg_match!") , } }
};
}
