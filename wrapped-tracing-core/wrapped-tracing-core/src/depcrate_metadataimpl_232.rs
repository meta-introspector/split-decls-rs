// Generated macro for impl_232 (impl)
macro_rules! Depcrate_metadataimpl_232 {
() => {
// Module: crate::metadata
// Provides: {"impl_232"}
// Dependencies: {}
impl fmt :: Debug for Metadata < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut meta = f . debug_struct ("Metadata") ; meta . field ("name" , & self . name) . field ("target" , & self . target) . field ("level" , & self . level) ; if let Some (path) = self . module_path () { meta . field ("module_path" , & path) ; } match (self . file () , self . line ()) { (Some (file) , Some (line)) => { meta . field ("location" , & format_args ! ("{}:{}" , file , line)) ; } (Some (file) , None) => { meta . field ("file" , & format_args ! ("{}" , file)) ; } (None , Some (line)) => { meta . field ("line" , & line) ; } (None , None) => { } } ; meta . field ("fields" , & format_args ! ("{}" , self . fields)) . field ("callsite" , & self . callsite ()) . field ("kind" , & self . kind) . finish () } }
};
}
