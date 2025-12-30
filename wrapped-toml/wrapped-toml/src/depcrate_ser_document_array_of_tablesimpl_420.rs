// Generated macro for impl_420 (impl)
macro_rules! Depcrate_ser_document_array_of_tablesimpl_420 {
() => {
// Module: crate::ser::document::array_of_tables
// Provides: {"impl_420"}
// Dependencies: {}
impl < 'd > ArrayOfTablesSerializer < 'd > { # [doc = " Creates a new serializer which will emit TOML into the buffer provided."] # [doc = ""] # [doc = " The serializer can then be used to serialize a type after which the data"] # [doc = " will be present in `dst`."] pub (crate) fn new (buf : & 'd mut Buffer , parent : Table , key : String , style : Style) -> Self { Self { buf , parent , key , style , } } }
};
}
