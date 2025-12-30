// Generated macro for impl_307 (impl)
macro_rules! Depcrate_backtraceimpl_307 {
() => {
// Module: crate::backtrace
// Provides: {"impl_307"}
// Dependencies: {}
impl fmt :: Debug for BacktraceSymbol { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{{ ") ? ; if let Some (fn_name) = self . name . as_ref () . map (| b | backtrace_rs :: SymbolName :: new (b)) { write ! (fmt , "fn: \"{:#}\"" , fn_name) ? ; } else { write ! (fmt , "fn: <unknown>") ? ; } if let Some (fname) = self . filename . as_ref () { write ! (fmt , ", file: \"{:?}\"" , fname) ? ; } if let Some (line) = self . lineno { write ! (fmt , ", line: {:?}" , line) ? ; } write ! (fmt , " }}") } }
};
}
