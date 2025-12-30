// Generated macro for impl_216 (impl)
macro_rules! Depcrate_subject_nameimpl_216 {
() => {
// Module: crate::subject_name
// Provides: {"impl_216"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Debug for GeneralName < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { GeneralName :: DnsName (name) => write ! (f , "DnsName(\"{}\")" , String :: from_utf8_lossy (name . as_slice_less_safe ())) , GeneralName :: DirectoryName => write ! (f , "DirectoryName") , GeneralName :: IpAddress (ip) => { write ! (f , "IpAddress({:?})" , IpAddrSlice (ip . as_slice_less_safe ())) } GeneralName :: UniformResourceIdentifier (uri) => write ! (f , "UniformResourceIdentifier(\"{}\")" , String :: from_utf8_lossy (uri . as_slice_less_safe ())) , GeneralName :: Unsupported (tag) => write ! (f , "Unsupported(0x{tag:02x})") , } } }
};
}
