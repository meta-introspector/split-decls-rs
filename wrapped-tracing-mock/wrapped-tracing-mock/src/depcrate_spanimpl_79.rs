// Generated macro for impl_79 (impl)
macro_rules! Depcrate_spanimpl_79 {
() => {
// Module: crate::span
// Provides: {"impl_79"}
// Dependencies: {}
impl fmt :: Display for SetActualSpanIdError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Could not set `ExpecedId` to {new}, \
            it had already been set to {previous}" , new = self . new_span_id , previous = self . previous_span_id) } }
};
}
