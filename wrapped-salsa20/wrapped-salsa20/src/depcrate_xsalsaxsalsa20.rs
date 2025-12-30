// Generated macro for XSalsa20 (type)
macro_rules! Depcrate_xsalsaXSalsa20 {
() => {
// Module: crate::xsalsa
// Provides: {"XSalsa20"}
// Dependencies: {}
# [doc = " XSalsa20 is a Salsa20 variant with an extended 192-bit (24-byte) nonce."] # [doc = ""] # [doc = " Based on the paper \"Extending the Salsa20 Nonce\":"] # [doc = ""] # [doc = " <https://cr.yp.to/snuffle/xsalsa-20081128.pdf>"] pub type XSalsa20 = StreamCipherCoreWrapper < XSalsaCore < U10 > > ;
};
}
