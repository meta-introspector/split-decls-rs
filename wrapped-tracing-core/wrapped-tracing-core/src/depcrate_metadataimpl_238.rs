// Generated macro for impl_238 (impl)
macro_rules! Depcrate_metadataimpl_238 {
() => {
// Module: crate::metadata
// Provides: {"impl_238"}
// Dependencies: {}
impl fmt :: Display for Level { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Level :: TRACE => f . pad ("TRACE") , Level :: DEBUG => f . pad ("DEBUG") , Level :: INFO => f . pad ("INFO") , Level :: WARN => f . pad ("WARN") , Level :: ERROR => f . pad ("ERROR") , } } }
};
}
