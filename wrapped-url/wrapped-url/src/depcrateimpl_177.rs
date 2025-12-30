// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
# [doc = " Debug the serialization of this URL."] impl fmt :: Debug for Url { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("Url") . field ("scheme" , & self . scheme ()) . field ("cannot_be_a_base" , & self . cannot_be_a_base ()) . field ("username" , & self . username ()) . field ("password" , & self . password ()) . field ("host" , & self . host ()) . field ("port" , & self . port ()) . field ("path" , & self . path ()) . field ("query" , & self . query ()) . field ("fragment" , & self . fragment ()) . finish () } }
};
}
