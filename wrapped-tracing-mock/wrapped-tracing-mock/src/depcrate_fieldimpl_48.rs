// Generated macro for impl_48 (impl)
macro_rules! Depcrate_fieldimpl_48 {
() => {
// Module: crate::field
// Provides: {"impl_48"}
// Dependencies: {}
impl fmt :: Display for ExpectedFields { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "fields ") ? ; let entries = self . fields . iter () . map (| (k , v) | (field :: display (k) , field :: display (v))) ; f . debug_map () . entries (entries) . finish () } }
};
}
