// Generated macro for impl_37 (impl)
macro_rules! Depcrate_printerimpl_37 {
() => {
// Module: crate::printer
// Provides: {"impl_37"}
// Dependencies: {}
impl < F , W > Printer < F , W > where F : 'static + Formatter , W : 'static + for < 'a > MakeWriter < 'a > , { # [doc = " Set the formatter."] # [doc = ""] # [doc = " See the [`Formatter`] trait for details on possible inputs."] pub fn formatter < F2 > (self , formatter : F2) -> Printer < F2 , W > where F2 : 'static + Formatter , { Printer { formatter , make_writer : self . make_writer , } } # [doc = " Set the writer."] pub fn writer < W2 > (self , make_writer : W2) -> Printer < F , W2 > where W2 : 'static + for < 'a > MakeWriter < 'a > , { Printer { formatter : self . formatter , make_writer , } } }
};
}
