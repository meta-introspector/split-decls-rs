// Generated macro for impl_432 (impl)
macro_rules! Depcrate_ser_document_bufferimpl_432 {
() => {
// Module: crate::ser::document::buffer
// Provides: {"impl_432"}
// Dependencies: {}
impl core :: fmt :: Display for Buffer { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut tables = self . tables . iter () . filter_map (| t | t . as_ref ()) . filter (| t | required_table (t)) ; if let Some (table) = tables . next () { table . fmt (f) ? ; } for table in tables { f . newline () ? ; table . fmt (f) ? ; } Ok (()) } }
};
}
