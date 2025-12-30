// Generated macro for impl_246 (impl)
macro_rules! Depcrate_metadataimpl_246 {
() => {
// Module: crate::metadata
// Provides: {"impl_246"}
// Dependencies: {}
impl fmt :: Display for LevelFilter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { LevelFilter :: OFF => f . pad ("off") , LevelFilter :: ERROR => f . pad ("error") , LevelFilter :: WARN => f . pad ("warn") , LevelFilter :: INFO => f . pad ("info") , LevelFilter :: DEBUG => f . pad ("debug") , LevelFilter :: TRACE => f . pad ("trace") , } } }
};
}
