// Generated macro for Table (type)
macro_rules! Depcrate_tableTable {
() => {
// Module: crate::table
// Provides: {"Table"}
// Dependencies: {}
# [doc = " Type representing a TOML table, payload of the `Value::Table` variant."] # [doc = ""] # [doc = " By default it entries are stored in"] # [doc = " [lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)"] # [doc = " of the keys. Enable the `preserve_order` feature to store entries in the order they appear in"] # [doc = " the source file."] pub type Table = Map < String , Value > ;
};
}
