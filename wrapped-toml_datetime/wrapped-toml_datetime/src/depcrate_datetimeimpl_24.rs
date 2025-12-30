// Generated macro for impl_24 (impl)
macro_rules! Depcrate_datetimeimpl_24 {
() => {
// Module: crate::datetime
// Provides: {"impl_24"}
// Dependencies: {}
impl Token < '_ > { fn is (& self , kind : TokenKind) -> Result < () , DatetimeParseError > { if self . kind == kind { Ok (()) } else { Err (DatetimeParseError :: new ()) } } }
};
}
