// Generated macro for to_document (function)
macro_rules! Depcrate_serto_document {
() => {
// Module: crate::ser
// Provides: {"to_document"}
// Dependencies: {}
# [doc = " Serialize the given data structure into a TOML document."] # [doc = ""] # [doc = " This would allow custom formatting to be applied, mixing with format preserving edits, etc."] pub fn to_document < T > (value : & T) -> Result < crate :: DocumentMut , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let value = value . serialize (ValueSerializer :: new ()) ? ; let item = crate :: Item :: Value (value) ; let root = item . into_table () . map_err (| _ | Error :: UnsupportedType (None)) ? ; Ok (root . into ()) }
};
}
