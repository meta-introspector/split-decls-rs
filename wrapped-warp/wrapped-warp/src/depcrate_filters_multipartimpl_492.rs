// Generated macro for impl_492 (impl)
macro_rules! Depcrate_filters_multipartimpl_492 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_492"}
// Dependencies: {}
impl fmt :: Debug for Part { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = f . debug_struct ("Part") ; builder . field ("name" , & self . name ()) ; if let Some (ref filename) = self . part . file_name () { builder . field ("filename" , filename) ; } if let Some (ref mime) = self . part . content_type () { builder . field ("content_type" , mime) ; } builder . finish () } }
};
}
