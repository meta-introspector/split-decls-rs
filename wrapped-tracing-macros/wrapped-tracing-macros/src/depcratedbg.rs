// Generated macro for dbg (macro)
macro_rules! Depcratedbg {
() => {
// Module: crate
// Provides: {"dbg"}
// Dependencies: {}
# [doc = " Similar to the `std::dbg!` macro, but generates `tracing` events rather"] # [doc = " than printing to stdout."] # [doc = ""] # [doc = " By default, the verbosity level for the generated events is `DEBUG`, but"] # [doc = " this can be customized."] # [macro_export] macro_rules ! dbg { (target : $ target : expr , level : $ level : expr , $ ex : expr) => { { match $ ex { value => { $ crate :: tracing :: event ! (target : $ target , $ level , ? value , stringify ! ($ ex)) ; value } } } } ; (level : $ level : expr , $ ex : expr) => { $ crate :: dbg ! (target : module_path ! () , level : $ level , $ ex) } ; (target : $ target : expr , $ ex : expr) => { $ crate :: dbg ! (target : $ target , level : $ crate :: tracing :: Level :: DEBUG , $ ex) } ; ($ ex : expr) => { $ crate :: dbg ! (level : $ crate :: tracing :: Level :: DEBUG , $ ex) } ; }
};
}
