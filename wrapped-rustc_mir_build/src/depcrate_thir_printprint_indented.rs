// Generated macro for print_indented (macro)
macro_rules! Depcrate_thir_printprint_indented {
() => {
// Module: crate::thir::print
// Provides: {"print_indented"}
// Dependencies: {}
macro_rules ! print_indented { ($ writer : ident , $ s : expr , $ indent_lvl : expr) => { $ writer . indent ($ indent_lvl) ; writeln ! ($ writer , "{}" , $ s) . expect ("unable to write to ThirPrinter") ; } ; }
};
}
