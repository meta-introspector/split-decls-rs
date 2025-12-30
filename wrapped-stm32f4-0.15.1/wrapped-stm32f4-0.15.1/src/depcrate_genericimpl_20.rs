// Generated macro for impl_20 (impl)
macro_rules! Depcrate_genericimpl_20 {
() => {
// Module: crate::generic
// Provides: {"impl_20"}
// Dependencies: {}
impl < U , FI > FieldReaderRaw < U , FI > where U : Copy , { # [doc = " Creates a new instance of the reader."] # [allow (unused)] # [inline (always)] pub (crate) fn new (bits : U) -> Self { Self { bits , _reg : marker :: PhantomData , } } }
};
}
