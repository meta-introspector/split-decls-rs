// Generated macro for traverse (function)
macro_rules! Depcrate_macrostraverse {
() => {
// Module: crate::macros
// Provides: {"traverse"}
// Dependencies: {}
fn traverse < 'a > (root : & 'a mut Value , path : & [& str]) -> & 'a mut Value { let mut cur = root ; for & key in path { let cur1 = cur ; let cur2 = if cur1 . is_array () { cur1 . as_array_mut () . unwrap () . last_mut () . unwrap () } else { cur1 } ; if ! cur2 . is_table () { * cur2 = Value :: Table (Table :: new ()) ; } if ! cur2 . as_table () . unwrap () . contains_key (key) { let empty = Value :: Table (Table :: new ()) ; cur2 . as_table_mut () . unwrap () . insert (key . to_owned () , empty) ; } cur = cur2 . as_table_mut () . unwrap () . get_mut (key) . unwrap () ; } cur }
};
}
