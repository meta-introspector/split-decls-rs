// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_rustfmt_diffimpl_1245 {
() => {
// Module: crate::rustfmt_diff
// Provides: {"impl_1245"}
// Dependencies: {}
impl fmt :: Display for ModifiedLines { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for chunk in & self . chunks { writeln ! (f , "{} {} {}" , chunk . line_number_orig , chunk . lines_removed , chunk . lines . len ()) ? ; for line in & chunk . lines { writeln ! (f , "{line}") ? ; } } Ok (()) } }
};
}
