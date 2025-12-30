// Generated macro for ptrace_event (macro)
macro_rules! Depcrate_dataptrace_event {
() => {
// Module: crate::data
// Provides: {"ptrace_event"}
// Dependencies: {}
# [macro_export] macro_rules ! ptrace_event { ($ cause : expr $ (, $ a : expr $ (, $ b : expr $ (, $ c : expr) ?) ?) ?) => { $ crate :: data :: PtraceEvent { cause : $ cause , $ (a : $ a , $ (b : $ b , $ (c : $ c ,) ?) ?) ? .. Default :: default () } } }
};
}
