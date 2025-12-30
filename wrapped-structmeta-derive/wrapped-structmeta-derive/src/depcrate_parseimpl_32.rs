// Generated macro for impl_32 (impl)
macro_rules! Depcrate_parseimpl_32 {
() => {
// Module: crate::parse
// Provides: {"impl_32"}
// Dependencies: {}
impl Parse for ParseAttribute { fn parse (input : ParseStream) -> Result < Self > { let mut any = None ; let mut peek = None ; let mut terminated = None ; let mut dump = None ; let args = input . parse_terminated (ParseAttributeArg :: parse , Token ! [,]) ? ; for arg in args . into_iter () { match arg { ParseAttributeArg :: Any (kw_any) => any = any . or (Some (kw_any)) , ParseAttributeArg :: Peek (kw_peek) => peek = peek . or (Some (kw_peek)) , ParseAttributeArg :: Terminated (kw_terminated) => { terminated = terminated . or (Some (kw_terminated)) } ParseAttributeArg :: Dump (kw_dump) => dump = dump . or (Some (kw_dump)) , } } Ok (Self { any , peek , terminated , dump , }) } }
};
}
