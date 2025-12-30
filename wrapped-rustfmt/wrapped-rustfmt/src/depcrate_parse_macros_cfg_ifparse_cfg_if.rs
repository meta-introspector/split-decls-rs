// Generated macro for parse_cfg_if (function)
macro_rules! Depcrate_parse_macros_cfg_ifparse_cfg_if {
() => {
// Module: crate::parse::macros::cfg_if
// Provides: {"parse_cfg_if"}
// Dependencies: {}
pub (crate) fn parse_cfg_if < 'a > (psess : & 'a ParseSess , mac : & 'a ast :: MacCall ,) -> Result < Vec < ast :: Item > , & 'static str > { match catch_unwind (AssertUnwindSafe (| | parse_cfg_if_inner (psess , mac))) { Ok (Ok (items)) => Ok (items) , Ok (err @ Err (_)) => err , Err (..) => Err ("failed to parse cfg_if!") , } }
};
}
