// Generated macro for DeTable (type)
macro_rules! Depcrate_de_parser_detableDeTable {
() => {
// Module: crate::de::parser::detable
// Provides: {"DeTable"}
// Dependencies: {}
# [doc = " Type representing a TOML table, payload of the `Value::Table` variant."] # [doc = ""] # [doc = " By default it entries are stored in"] # [doc = " [lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)"] # [doc = " of the keys. Enable the `preserve_order` feature to store entries in the order they appear in"] # [doc = " the source file."] pub type DeTable < 'i > = Map < Spanned < DeString < 'i > > , Spanned < DeValue < 'i > > > ;
};
}
