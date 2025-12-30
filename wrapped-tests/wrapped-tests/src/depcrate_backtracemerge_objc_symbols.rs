// Generated macro for merge_objc_symbols (function)
macro_rules! Depcrate_backtracemerge_objc_symbols {
() => {
// Module: crate::backtrace
// Provides: {"merge_objc_symbols"}
// Dependencies: {}
# [allow (dead_code)] fn merge_objc_symbols (exc : & NSException) -> Vec < String > { let mut demangled_symbols = vec ! [] ; let nssymbols = exc . callStackSymbols () ; let return_addrs = exc . callStackReturnAddresses () ; for (nssymbol , addr) in nssymbols . iter () . zip (return_addrs) { let addr = addr . as_usize () as * mut c_void ; let mut call_count = 0 ; backtrace :: resolve (addr , | symbol | { if let Some (name) = symbol . name () { demangled_symbols . push (name . to_string ()) ; } else { demangled_symbols . push (format ! ("{nssymbol} ({:?})" , symbol . addr ())) ; } call_count += 1 ; }) ; if call_count == 0 { demangled_symbols . push (nssymbol . to_string ()) ; } } demangled_symbols }
};
}
