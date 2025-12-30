// Generated macro for xxh3_template (macro)
macro_rules! Depcratexxh3_template {
() => {
// Module: crate
// Provides: {"xxh3_template"}
// Dependencies: {}
# [doc = " Constructs a wrapper around the XXH3_* familiy of functions as we"] # [doc = " compile the library in multiple modes to performance test against."] macro_rules ! xxh3_template { () => { crate :: xxh3_template ! (@ XXH3) ; } ; ($ prefix : ident) => { :: paste :: paste ! { crate :: xxh3_template ! (@ [< $ prefix _XXH3 >]) ; } } ; (@ $ prefix : ident) => { :: paste :: paste ! { extern "C" { fn [<$ prefix _createState >] () -> * mut crate :: XXH3_state_t ; fn [<$ prefix _freeState >] (state : * mut crate :: XXH3_state_t) -> crate :: XXH_errorcode ; } } } ; }
};
}
