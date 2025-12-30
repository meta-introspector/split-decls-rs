// Generated macro for parse_quote (macro)
macro_rules! Depcrateparse_quote {
() => {
// Module: crate
// Provides: {"parse_quote"}
// Dependencies: {}
macro_rules ! parse_quote { ($ ($ inp : tt) *) => { { let tt = quote ! { $ ($ inp) * } ; syn :: parse2 (tt . clone ()) . unwrap_or_else (| err | { panic ! ("failed to parse `{}` at {}:{}:{}: {}" , tt , file ! () , line ! () , column ! () , err) }) } } }
};
}
