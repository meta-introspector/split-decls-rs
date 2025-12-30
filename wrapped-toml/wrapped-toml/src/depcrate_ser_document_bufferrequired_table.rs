// Generated macro for required_table (function)
macro_rules! Depcrate_ser_document_bufferrequired_table {
() => {
// Module: crate::ser::document::buffer
// Provides: {"required_table"}
// Dependencies: {}
fn required_table (table : & Table) -> bool { if table . key . is_none () { ! table . body . is_empty () } else { table . array || ! table . body . is_empty () || ! table . has_children } }
};
}
