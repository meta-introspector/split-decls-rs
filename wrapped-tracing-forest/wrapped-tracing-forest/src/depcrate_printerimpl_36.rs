// Generated macro for impl_36 (impl)
macro_rules! Depcrate_printerimpl_36 {
() => {
// Module: crate::printer
// Provides: {"impl_36"}
// Dependencies: {}
impl PrettyPrinter { # [doc = " Returns a new [`PrettyPrinter`] that pretty-prints to stdout."] # [doc = ""] # [doc = " Use [`Printer::formatter`] and [`Printer::writer`] for custom configuration."] pub const fn new () -> Self { Printer { formatter : Pretty , make_writer : MakeStdout , } } }
};
}
