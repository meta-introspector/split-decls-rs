// Generated macro for impl_37 (impl)
macro_rules! Depcrate_genericimpl_37 {
() => {
// Module: crate::generic
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , U , REG , N , FI , const WI : u8 , const OF : u8 > FieldWriter < 'a , U , REG , N , FI , WI , OF > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < N > , { # [doc = " Field width"] pub const WIDTH : u8 = WI ; # [doc = " Field offset"] pub const OFFSET : u8 = OF ; }
};
}
