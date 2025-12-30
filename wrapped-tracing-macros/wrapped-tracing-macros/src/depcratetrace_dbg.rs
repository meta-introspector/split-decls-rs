// Generated macro for trace_dbg (macro)
macro_rules! Depcratetrace_dbg {
() => {
// Module: crate
// Provides: {"trace_dbg"}
// Dependencies: {}
# [doc = " Alias of `dbg!` for avoiding conflicts with the `std::dbg!` macro."] # [macro_export] macro_rules ! trace_dbg { (target : $ target : expr , level : $ level : expr , $ ex : expr) => { $ crate :: dbg ! (target : $ target , level : $ level , $ ex) } ; (level : $ level : expr , $ ex : expr) => { $ crate :: dbg ! (target : module_path ! () , level : $ level , $ ex) } ; (target : $ target : expr , $ ex : expr) => { $ crate :: dbg ! (target : $ target , level : $ crate :: tracing :: Level :: DEBUG , $ ex) } ; ($ ex : expr) => { $ crate :: dbg ! (level : $ crate :: tracing :: Level :: DEBUG , $ ex) } ; }
};
}
