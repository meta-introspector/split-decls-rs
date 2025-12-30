// Generated macro for tokens (macro)
macro_rules! Depcrate_tokenstokens {
() => {
// Module: crate::tokens
// Provides: {"tokens"}
// Dependencies: {}
macro_rules ! tokens { (ops : { $ (($ ($ op : tt) *) ,) * } delim : { $ (($ ($ delim : tt) *) ,) * } syms : { $ (($ ($ sym : tt) *) ,) * }) => ($ (op ! { $ ($ op) * }) * $ (delim ! { $ ($ delim) * }) * $ (sym ! { $ ($ sym) * }) *) }
};
}
