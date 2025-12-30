// Generated macro for impl_715 (impl)
macro_rules! Depcrate_teraimpl_715 {
() => {
// Module: crate::tera
// Provides: {"impl_715"}
// Dependencies: {}
impl fmt :: Debug for Tera { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Tera {{") ? ; writeln ! (f , "\n\ttemplates: [") ? ; for template in self . templates . keys () { writeln ! (f , "\t\t{}," , template) ? ; } write ! (f , "\t]") ? ; writeln ! (f , "\n\tfilters: [") ? ; for filter in self . filters . keys () { writeln ! (f , "\t\t{}," , filter) ? ; } write ! (f , "\t]") ? ; writeln ! (f , "\n\ttesters: [") ? ; for tester in self . testers . keys () { writeln ! (f , "\t\t{}," , tester) ? ; } writeln ! (f , "\t]") ? ; writeln ! (f , "}}") } }
};
}
