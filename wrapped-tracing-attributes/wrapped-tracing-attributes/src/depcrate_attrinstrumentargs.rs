// Generated macro for InstrumentArgs (struct)
macro_rules! Depcrate_attrInstrumentArgs {
() => {
// Module: crate::attr
// Provides: {"InstrumentArgs"}
// Dependencies: {}
# [derive (Clone , Default , Debug)] pub (crate) struct InstrumentArgs { level : Option < Level > , pub (crate) name : Option < LitStrOrIdent > , target : Option < LitStrOrIdent > , pub (crate) parent : Option < Expr > , pub (crate) follows_from : Option < Expr > , pub (crate) skips : HashSet < Ident > , pub (crate) skip_all : bool , pub (crate) fields : Option < Fields > , pub (crate) err_args : Option < EventArgs > , pub (crate) ret_args : Option < EventArgs > , # [doc = " Errors describing any unrecognized parse inputs that we skipped."] parse_warnings : Vec < syn :: Error > , }
};
}
