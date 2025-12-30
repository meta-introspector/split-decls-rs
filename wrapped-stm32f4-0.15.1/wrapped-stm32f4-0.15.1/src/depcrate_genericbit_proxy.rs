// Generated macro for bit_proxy (macro)
macro_rules! Depcrate_genericbit_proxy {
() => {
// Module: crate::generic
// Provides: {"bit_proxy"}
// Dependencies: {}
macro_rules ! bit_proxy { ($ writer : ident , $ mwv : ident) => { # [doc (hidden)] pub struct $ mwv ; # [doc = " Bit-wise write field proxy"] pub type $ writer <'a , U , REG , FI , const O : u8 > = BitWriterRaw <'a , U , REG , FI , $ mwv , O >; impl <'a , U , REG , FI , const OF : u8 > $ writer <'a , U , REG , FI , OF > where REG : Writable + RegisterSpec < Ux = U >, FI : Into < bool >, { # [doc = " Field width"] pub const WIDTH : u8 = 1 ; # [doc = " Field offset"] pub const OFFSET : u8 = OF ; } } }
};
}
