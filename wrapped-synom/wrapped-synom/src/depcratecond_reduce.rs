// Generated macro for cond_reduce (macro)
macro_rules! Depcratecond_reduce {
() => {
// Module: crate
// Provides: {"cond_reduce"}
// Dependencies: {}
# [doc = " Fail to parse if condition is false, otherwise parse the given parser."] # [doc = ""] # [doc = " This is typically used inside of `option!` or `alt!`."] # [doc = ""] # [doc = " - **Syntax:** `cond_reduce!(CONDITION, THING)`"] # [doc = " - **Output:** `THING`"] # [macro_export] macro_rules ! cond_reduce { ($ i : expr , $ cond : expr , $ submac : ident ! ($ ($ args : tt) *)) => { if $ cond { $ submac ! ($ i , $ ($ args) *) } else { $ crate :: parse_error () } } ; ($ i : expr , $ cond : expr , $ f : expr) => { cond_reduce ! ($ i , $ cond , call ! ($ f)) } ; }
};
}
