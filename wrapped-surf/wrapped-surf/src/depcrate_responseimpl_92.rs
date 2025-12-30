// Generated macro for impl_92 (impl)
macro_rules! Depcrate_responseimpl_92 {
() => {
// Module: crate::response
// Provides: {"impl_92"}
// Dependencies: {}
impl fmt :: Debug for DecodeError { # [allow (missing_doc_code_examples)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DecodeError") . field ("encoding" , & self . encoding) . field ("data" , & format ! ("{} bytes" , self . data . len ())) . finish () } }
};
}
