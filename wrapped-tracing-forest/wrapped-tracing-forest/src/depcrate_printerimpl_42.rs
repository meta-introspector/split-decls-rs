// Generated macro for impl_42 (impl)
macro_rules! Depcrate_printerimpl_42 {
() => {
// Module: crate::printer
// Provides: {"impl_42"}
// Dependencies: {}
impl < F > Processor for TestCapturePrinter < F > where F : 'static + Formatter , { fn process (& self , tree : Tree) -> processor :: Result { let string = self . formatter . fmt (& tree) . map_err (| e | processor :: error (tree , e . into ())) ? ; print ! ("{string}") ; Ok (()) } }
};
}
