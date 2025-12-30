// Generated macro for test_println (macro)
macro_rules! Depcrate_macrostest_println {
() => {
// Module: crate::macros
// Provides: {"test_println"}
// Dependencies: {}
macro_rules ! test_println { ($ ($ arg : tt) *) => { if cfg ! (test) && cfg ! (slab_print) { if std :: thread :: panicking () { println ! ("[PANIC {:>17}:{:<3}] {}" , file ! () , line ! () , format_args ! ($ ($ arg) *)) } else { println ! ("[{:?} {:>17}:{:<3}] {}" , crate :: Tid ::< crate :: DefaultConfig >:: current () , file ! () , line ! () , format_args ! ($ ($ arg) *)) } } } }
};
}
