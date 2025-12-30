// Generated macro for s (macro)
macro_rules! Depcrate_literalss {
() => {
// Module: crate::literals
// Provides: {"s"}
// Dependencies: {}
# [doc = " A literal UTF-8 string with a trailing null terminator."] # [macro_export] macro_rules ! s { ($ s : literal) => { $ crate :: PCSTR :: from_raw (:: core :: concat ! ($ s , '\0') . as_ptr ()) } ; }
};
}
