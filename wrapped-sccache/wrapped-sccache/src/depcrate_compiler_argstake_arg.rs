// Generated macro for take_arg (macro)
macro_rules! Depcrate_compiler_argstake_arg {
() => {
// Module: crate::compiler::args
// Provides: {"take_arg"}
// Dependencies: {}
# [doc = " Helper macro used to define ArgInfo::TakeArg's."] # [doc = " Variant is an enum variant, e.g. enum ArgType { Variant(OsString) }"] # [doc = "     take_arg!(\"-foo\", OsString, Separated, Variant)"] # [doc = "     take_arg!(\"-foo\", OsString, Concatenated, Variant)"] # [doc = "     take_arg!(\"-foo\", OsString, Concatenated(b'='), Variant)"] macro_rules ! take_arg { ($ s : expr , $ vtype : ident , Separated , $ variant : expr) => { ArgInfo :: TakeArg ($ s , | arg : OsString | $ vtype :: process (arg) . map ($ variant) , ArgDisposition :: Separated ,) } ; ($ s : expr , $ vtype : ident , $ d : ident , $ variant : expr) => { ArgInfo :: TakeArg ($ s , | arg : OsString | $ vtype :: process (arg) . map ($ variant) , ArgDisposition ::$ d (None) ,) } ; ($ s : expr , $ vtype : ident , $ d : ident ($ x : expr) , $ variant : expr) => { ArgInfo :: TakeArg ($ s , | arg : OsString | $ vtype :: process (arg) . map ($ variant) , ArgDisposition ::$ d (Some ($ x)) ,) } ; }
};
}
