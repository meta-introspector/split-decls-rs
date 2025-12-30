// Generated macro for impl_446 (impl)
macro_rules! Depcrate_deimpl_446 {
() => {
// Module: crate::de
// Provides: {"impl_446"}
// Dependencies: {}
# [cfg (feature = "parse")] impl std :: str :: FromStr for Deserializer { type Err = Error ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let doc : crate :: Document < _ > = s . parse () . map_err (Error :: from) ? ; Ok (Self :: from (doc)) } }
};
}
