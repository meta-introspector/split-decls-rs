// Generated macro for impl_133 (impl)
macro_rules! Depcrate_ir_printimpl_133 {
() => {
// Module: crate::ir_print
// Provides: {"impl_133"}
// Dependencies: {}
impl < I : Interner , T > fmt :: Display for Binder < I , T > where I : IrPrint < Binder < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < Binder < I , T > > > :: print (self , fmt) } }
};
}
