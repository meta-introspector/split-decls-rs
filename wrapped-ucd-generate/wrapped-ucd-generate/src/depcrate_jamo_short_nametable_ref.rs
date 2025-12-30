// Generated macro for table_ref (function)
macro_rules! Depcrate_jamo_short_nametable_ref {
() => {
// Module: crate::jamo_short_name
// Provides: {"table_ref"}
// Dependencies: {}
pub fn table_ref < 'a > (table : & 'a [(u32 , String)]) -> Vec < (u32 , & 'a str) > { table . iter () . map (| & (cp , ref name) | (cp , & * * name)) . collect () }
};
}
