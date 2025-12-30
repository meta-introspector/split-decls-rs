// Generated macro for format_extern (function)
macro_rules! Depcrate_utilsformat_extern {
() => {
// Module: crate::utils
// Provides: {"format_extern"}
// Dependencies: {}
# [inline] pub (crate) fn format_extern (ext : ast :: Extern , explicit_abi : bool) -> Cow < 'static , str > { match ext { ast :: Extern :: None => Cow :: from ("") , ast :: Extern :: Implicit (_) if explicit_abi => Cow :: from ("extern \"C\" ") , ast :: Extern :: Implicit (_) => Cow :: from ("extern ") , ast :: Extern :: Explicit (abi , _) if abi . symbol_unescaped == sym :: C && ! explicit_abi => { Cow :: from ("extern ") } ast :: Extern :: Explicit (abi , _) => { Cow :: from (format ! (r#"extern "{}" "# , abi . symbol_unescaped)) } } }
};
}
