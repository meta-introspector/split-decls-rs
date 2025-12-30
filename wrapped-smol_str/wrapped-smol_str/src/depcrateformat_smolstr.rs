// Generated macro for format_smolstr (macro)
macro_rules! Depcrateformat_smolstr {
() => {
// Module: crate
// Provides: {"format_smolstr"}
// Dependencies: {}
# [doc = " Formats arguments to a [`SmolStr`], potentially without allocating."] # [doc = ""] # [doc = " See [`alloc::format!`] or [`format_args!`] for syntax documentation."] # [macro_export] macro_rules ! format_smolstr { ($ ($ tt : tt) *) => { { let mut w = $ crate :: SmolStrBuilder :: new () ; :: core :: fmt :: Write :: write_fmt (& mut w , format_args ! ($ ($ tt) *)) . expect ("a formatting trait implementation returned an error") ; w . finish () } } ; }
};
}
