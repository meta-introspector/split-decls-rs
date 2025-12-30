// Generated macro for impl_1914 (impl)
macro_rules! Depcrate_enumsimpl_1914 {
() => {
// Module: crate::enums
// Provides: {"impl_1914"}
// Dependencies: {}
impl HashAlgorithm { # [doc = " Returns the hash of the empty input."] # [doc = ""] # [doc = " This returns `None` for some hash algorithms, so the caller"] # [doc = " should be prepared to do the computation themselves in this case."] pub (crate) fn hash_for_empty_input (& self) -> Option < hash :: Output > { match self { Self :: SHA256 => Some (hash :: Output :: new (b"\xe3\xb0\xc4\x42\x98\xfc\x1c\x14\
                  \x9a\xfb\xf4\xc8\x99\x6f\xb9\x24\
                  \x27\xae\x41\xe4\x64\x9b\x93\x4c\
                  \xa4\x95\x99\x1b\x78\x52\xb8\x55" ,)) , Self :: SHA384 => Some (hash :: Output :: new (b"\x38\xb0\x60\xa7\x51\xac\x96\x38\
                  \x4c\xd9\x32\x7e\xb1\xb1\xe3\x6a\
                  \x21\xfd\xb7\x11\x14\xbe\x07\x43\
                  \x4c\x0c\xc7\xbf\x63\xf6\xe1\xda\
                  \x27\x4e\xde\xbf\xe7\x6f\x65\xfb\
                  \xd5\x1a\xd2\xf1\x48\x98\xb9\x5b" ,)) , _ => None , } } }
};
}
