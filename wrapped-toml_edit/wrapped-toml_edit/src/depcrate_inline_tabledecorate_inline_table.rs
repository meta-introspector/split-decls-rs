// Generated macro for decorate_inline_table (function)
macro_rules! Depcrate_inline_tabledecorate_inline_table {
() => {
// Module: crate::inline_table
// Provides: {"decorate_inline_table"}
// Dependencies: {}
fn decorate_inline_table (table : & mut InlineTable) { use indexmap :: map :: MutableKeys ; for (mut key , value) in table . items . iter_mut2 () . filter (| (_ , value) | value . is_value ()) . map (| (key , value) | (key . as_mut () , value . as_value_mut () . unwrap ())) { key . leaf_decor_mut () . clear () ; key . dotted_decor_mut () . clear () ; value . decor_mut () . clear () ; } }
};
}
