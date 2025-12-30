// Generated macro for impl_137 (impl)
macro_rules! Depcrate_ir_printimpl_137 {
() => {
// Module: crate::ir_print
// Provides: {"impl_137"}
// Dependencies: {}
impl < I : Interner , T > fmt :: Display for OutlivesPredicate < I , T > where I : IrPrint < OutlivesPredicate < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < OutlivesPredicate < I , T > > > :: print (self , fmt) } }
};
}
