// Generated macro for Table (struct)
macro_rules! Depcrate_tableTable {
() => {
// Module: crate::table
// Provides: {"Table"}
// Dependencies: {}
# [doc = " A TOML table, a top-level collection of key/[`Value`] pairs under a header and logical"] # [doc = " sub-tables"] # [derive (Clone , Debug , Default)] pub struct Table { pub (crate) decor : Decor , pub (crate) implicit : bool , pub (crate) dotted : bool , doc_position : Option < isize > , pub (crate) span : Option < std :: ops :: Range < usize > > , pub (crate) items : KeyValuePairs , }
};
}
