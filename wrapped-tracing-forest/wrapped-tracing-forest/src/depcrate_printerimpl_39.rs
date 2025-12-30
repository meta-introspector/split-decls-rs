// Generated macro for impl_39 (impl)
macro_rules! Depcrate_printerimpl_39 {
() => {
// Module: crate::printer
// Provides: {"impl_39"}
// Dependencies: {}
impl < F , W > Processor for Printer < F , W > where F : 'static + Formatter , W : 'static + for < 'a > MakeWriter < 'a > , { fn process (& self , tree : Tree) -> processor :: Result { let string = match self . formatter . fmt (& tree) { Ok (s) => s , Err (e) => return Err (processor :: error (tree , e . into ())) , } ; match self . make_writer . make_writer () . write_all (string . as_bytes ()) { Ok (()) => Ok (()) , Err (e) => Err (processor :: error (tree , e . into ())) , } } }
};
}
