// Generated macro for value (function)
macro_rules! Depcrate_itemvalue {
() => {
// Module: crate::item
// Provides: {"value"}
// Dependencies: {}
# [doc = " Returns a formatted value."] # [doc = ""] # [doc = " Since formatting is part of a `Value`, the right hand side of the"] # [doc = " assignment needs to be decorated with a space before the value."] # [doc = " The `value` function does just that."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"display\")] {"] # [doc = " # #[cfg(feature = \"parse\")] {"] # [doc = " # use toml_edit::*;"] # [doc = " let mut table = Table::default();"] # [doc = " let mut array = Array::default();"] # [doc = " array.push(\"hello\");"] # [doc = " array.push(\"\\\\, world\"); // \\ is only allowed in a literal string"] # [doc = " table[\"key1\"] = value(\"value1\");"] # [doc = " table[\"key2\"] = value(42);"] # [doc = " table[\"key3\"] = value(array);"] # [doc = " assert_eq!(table.to_string(),"] # [doc = " r#\"key1 = \"value1\""] # [doc = " key2 = 42"] # [doc = " key3 = [\"hello\", '\\, world']"] # [doc = " \"#);"] # [doc = " # }"] # [doc = " # }"] # [doc = " ```"] pub fn value < V : Into < Value > > (v : V) -> Item { Item :: Value (v . into ()) }
};
}
