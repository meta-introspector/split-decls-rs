// Generated macro for from_slice (function)
macro_rules! Depcrate_defrom_slice {
() => {
// Module: crate::de
// Provides: {"from_slice"}
// Dependencies: {}
# [doc = " Deserializes bytes into a type."] # [doc = ""] # [doc = " This function will attempt to interpret `s` as a TOML document and"] # [doc = " deserialize `T` from the document."] # [doc = ""] # [doc = " To deserializes TOML values, instead of documents, see [`ValueDeserializer`]."] # [cfg (feature = "parse")] pub fn from_slice < T > (s : & '_ [u8]) -> Result < T , Error > where T : DeserializeOwned , { let s = std :: str :: from_utf8 (s) . map_err (| e | Error :: custom (e , None)) ? ; from_str (s) }
};
}
