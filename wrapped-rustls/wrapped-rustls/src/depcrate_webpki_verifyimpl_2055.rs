// Generated macro for impl_2055 (impl)
macro_rules! Depcrate_webpki_verifyimpl_2055 {
() => {
// Module: crate::webpki::verify
// Provides: {"impl_2055"}
// Dependencies: {}
impl fmt :: Debug for WebPkiSupportedAlgorithms { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "WebPkiSupportedAlgorithms {{ all: [ .. ], mapping: ") ? ; f . debug_list () . entries (self . mapping . iter () . map (| item | item . 0)) . finish () ? ; write ! (f , " }}") } }
};
}
