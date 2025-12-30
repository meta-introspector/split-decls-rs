// Generated macro for impl_bit_proxy (macro)
macro_rules! Depcrate_genericimpl_bit_proxy {
() => {
// Module: crate::generic
// Provides: {"impl_bit_proxy"}
// Dependencies: {}
macro_rules ! impl_bit_proxy { ($ writer : ident , $ U : ty) => { impl <'a , REG , FI , const OF : u8 > $ writer <'a , $ U , REG , FI , OF > where REG : Writable + RegisterSpec < Ux = $ U >, FI : Into < bool >, { # [doc = " Writes bit to the field"] # [inline (always)] pub fn bit (self , value : bool) -> &'a mut REG :: Writer { self . w . bits = (self . w . bits & ! (1 << { OF })) | ((<$ U >:: from (value) & 1) << { OF }) ; self . w } # [doc = " Writes `variant` to the field"] # [inline (always)] pub fn variant (self , variant : FI) -> &'a mut REG :: Writer { self . bit (variant . into ()) } } } }
};
}
