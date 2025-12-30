// Generated macro for impl_38 (impl)
macro_rules! Depcrate_genericimpl_38 {
() => {
// Module: crate::generic
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , U , REG , N , FI , const WI : u8 , const OF : u8 > FieldWriterSafe < 'a , U , REG , N , FI , WI , OF > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < N > , { # [doc = " Field width"] pub const WIDTH : u8 = WI ; # [doc = " Field offset"] pub const OFFSET : u8 = OF ; }
};
}
