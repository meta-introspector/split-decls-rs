// Generated macro for impl_47 (impl)
macro_rules! Depcrate_documentimpl_47 {
() => {
// Module: crate::document
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "parse")] impl FromStr for Document < String > { type Err = crate :: TomlError ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: parse (s . to_owned ()) } }
};
}
