// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl Error { pub (crate) fn report (& self) { let current_error : & dyn std :: error :: Error = self ; let mut current_error = Some (current_error) ; let mut ind = 0 ; eprintln ! ("Error:") ; while let Some (error) = current_error { eprintln ! ("    {}: {}" , ind , error) ; ind += 1 ; current_error = error . source () ; } } }
};
}
