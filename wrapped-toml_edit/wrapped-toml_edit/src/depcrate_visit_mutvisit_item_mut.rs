// Generated macro for visit_item_mut (function)
macro_rules! Depcrate_visit_mutvisit_item_mut {
() => {
// Module: crate::visit_mut
// Provides: {"visit_item_mut"}
// Dependencies: {}
pub fn visit_item_mut < V > (v : & mut V , node : & mut Item) where V : VisitMut + ? Sized , { match node { Item :: None => { } Item :: Value (value) => v . visit_value_mut (value) , Item :: Table (table) => v . visit_table_mut (table) , Item :: ArrayOfTables (array) => v . visit_array_of_tables_mut (array) , } }
};
}
