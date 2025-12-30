// Generated macro for impl_436 (impl)
macro_rules! Depcrate_ser_document_bufferimpl_436 {
() => {
// Module: crate::ser::document::buffer
// Provides: {"impl_436"}
// Dependencies: {}
impl core :: fmt :: Display for Table { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if let Some (key) = & self . key { if self . array { f . open_array_of_tables_header () ? ; } else { f . open_table_header () ? ; } let mut key = key . iter () ; if let Some (key) = key . next () { write ! (f , "{key}") ? ; } for key in key { f . key_sep () ? ; write ! (f , "{key}") ? ; } if self . array { f . close_array_of_tables_header () ? ; } else { f . close_table_header () ? ; } f . newline () ? ; } self . body . fmt (f) ? ; Ok (()) } }
};
}
