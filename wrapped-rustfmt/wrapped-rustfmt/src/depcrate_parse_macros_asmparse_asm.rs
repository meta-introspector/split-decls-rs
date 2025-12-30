// Generated macro for parse_asm (function)
macro_rules! Depcrate_parse_macros_asmparse_asm {
() => {
// Module: crate::parse::macros::asm
// Provides: {"parse_asm"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn parse_asm (context : & RewriteContext < '_ > , mac : & ast :: MacCall) -> Option < Vec < AsmArg > > { let ts = mac . args . tokens . clone () ; let mut parser = super :: build_parser (context , ts) ; parse_asm_args (& mut parser , mac . span () , ast :: AsmMacro :: Asm) . ok () }
};
}
