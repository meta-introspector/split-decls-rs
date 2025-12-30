// Generated macro for visit_nested_tables (function)
macro_rules! Depcrate_encodevisit_nested_tables {
() => {
// Module: crate::encode
// Provides: {"visit_nested_tables"}
// Dependencies: {}
fn visit_nested_tables < 't , F > (table : & 't Table , path : & mut Vec < Key > , is_array_of_tables : bool , callback : & mut F ,) -> Result where F : FnMut (& 't Table , & Vec < Key > , bool) -> Result , { if ! table . is_dotted () { callback (table , path , is_array_of_tables) ? ; } for (key , value) in table . items . iter () { match value { Item :: Table (t) => { let key = key . clone () ; path . push (key) ; visit_nested_tables (t , path , false , callback) ? ; path . pop () ; } Item :: ArrayOfTables (a) => { for t in a . iter () { let key = key . clone () ; path . push (key) ; visit_nested_tables (t , path , true , callback) ? ; path . pop () ; } } _ => { } } } Ok (()) }
};
}
