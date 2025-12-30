// Generated macro for impl_423 (impl)
macro_rules! Depcrate_ser_document_array_of_tablesimpl_423 {
() => {
// Module: crate::ser::document::array_of_tables
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'd > SerializeArrayOfTablesSerializer < 'd > { pub (crate) fn seq (buf : & 'd mut Buffer , parent : Table , key : String , style : Style) -> Self { Self { buf , parent , key , style , } } fn end (self) -> Result < & 'd mut Buffer , Error > { Ok (self . buf) } }
};
}
