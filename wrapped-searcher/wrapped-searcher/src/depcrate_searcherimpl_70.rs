// Generated macro for impl_70 (impl)
macro_rules! Depcrate_searcherimpl_70 {
() => {
// Module: crate::searcher
// Provides: {"impl_70"}
// Dependencies: {}
impl Encoding { # [doc = " Create a new encoding for the specified label."] # [doc = ""] # [doc = " The encoding label provided is mapped to an encoding via the set of"] # [doc = " available choices specified in the"] # [doc = " [Encoding Standard](https://encoding.spec.whatwg.org/#concept-encoding-get)."] # [doc = " If the given label does not correspond to a valid encoding, then this"] # [doc = " returns an error."] pub fn new (label : & str) -> Result < Encoding , ConfigError > { let label = label . as_bytes () ; match encoding_rs :: Encoding :: for_label_no_replacement (label) { Some (encoding) => Ok (Encoding (encoding)) , None => { Err (ConfigError :: UnknownEncoding { label : label . to_vec () }) } } } }
};
}
