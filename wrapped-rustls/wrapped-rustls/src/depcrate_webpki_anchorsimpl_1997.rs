// Generated macro for impl_1997 (impl)
macro_rules! Depcrate_webpki_anchorsimpl_1997 {
() => {
// Module: crate::webpki::anchors
// Provides: {"impl_1997"}
// Dependencies: {}
impl fmt :: Debug for RootCertStore { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RootCertStore") . field ("roots" , & format ! ("({} roots)" , & self . roots . len ())) . finish () } }
};
}
