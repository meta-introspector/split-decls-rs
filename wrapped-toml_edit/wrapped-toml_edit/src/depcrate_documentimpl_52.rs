// Generated macro for impl_52 (impl)
macro_rules! Depcrate_documentimpl_52 {
() => {
// Module: crate::document
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "parse")] impl FromStr for DocumentMut { type Err = crate :: TomlError ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let im = Document :: from_str (s) ? ; Ok (im . into_mut ()) } }
};
}
