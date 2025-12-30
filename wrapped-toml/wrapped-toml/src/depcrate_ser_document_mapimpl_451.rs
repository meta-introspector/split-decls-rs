// Generated macro for impl_451 (impl)
macro_rules! Depcrate_ser_document_mapimpl_451 {
() => {
// Module: crate::ser::document::map
// Provides: {"impl_451"}
// Dependencies: {}
impl < 'd > SerializeDocumentTable < 'd > { pub (crate) fn map (buf : & 'd mut Buffer , table : Table , style : Style) -> Result < Self , Error > { Ok (Self { buf , table , key : None , style , }) } fn end (self) -> Result < & 'd mut Buffer , Error > { self . buf . push (self . table) ; Ok (self . buf) } }
};
}
