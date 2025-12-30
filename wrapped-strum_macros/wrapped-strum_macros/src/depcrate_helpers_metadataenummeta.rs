// Generated macro for EnumMeta (enum)
macro_rules! Depcrate_helpers_metadataEnumMeta {
() => {
// Module: crate::helpers::metadata
// Provides: {"EnumMeta"}
// Dependencies: {}
pub enum EnumMeta { SerializeAll { kw : kw :: serialize_all , case_style : CaseStyle , } , AsciiCaseInsensitive (kw :: ascii_case_insensitive) , Crate { kw : kw :: Crate , crate_module_path : Path , } , UsePhf (kw :: use_phf) , Prefix { kw : kw :: prefix , prefix : LitStr , } , Suffix { kw : kw :: suffix , suffix : LitStr , } , ParseErrTy { kw : kw :: parse_err_ty , path : Path , } , ParseErrFn { kw : kw :: parse_err_fn , path : Path , } , ConstIntoStr (kw :: const_into_str) , }
};
}
