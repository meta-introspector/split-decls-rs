// Generated macro for Item (enum)
macro_rules! Depcrate_itemItem {
() => {
// Module: crate::item
// Provides: {"Item"}
// Dependencies: {}
# [doc = " Type representing either a value, a table, an array of tables, or none."] # [derive (Debug , Default)] pub enum Item { # [doc = " Type representing none."] # [default] None , # [doc = " Type representing value."] Value (Value) , # [doc = " Type representing table."] Table (Table) , # [doc = " Type representing array of tables."] ArrayOfTables (ArrayOfTables) , }
};
}
