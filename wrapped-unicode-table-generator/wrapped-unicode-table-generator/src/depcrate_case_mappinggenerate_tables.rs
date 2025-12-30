// Generated macro for generate_tables (function)
macro_rules! Depcrate_case_mappinggenerate_tables {
() => {
// Module: crate::case_mapping
// Provides: {"generate_tables"}
// Dependencies: {}
fn generate_tables (case : & str , data : & BTreeMap < u32 , [u32 ; 3] >) -> (String , usize) { let mut mappings = Vec :: with_capacity (data . len ()) ; let mut multis = Vec :: new () ; for (& key , & [a , b , c]) in data . iter () { let key = char :: from_u32 (key) . unwrap () ; if key . is_ascii () { continue ; } let value = if b == 0 && c == 0 { a } else { multis . push ([CharEscape (char :: from_u32 (a) . unwrap ()) , CharEscape (char :: from_u32 (b) . unwrap ()) , CharEscape (char :: from_u32 (c) . unwrap ()) ,]) ; INDEX_MASK | (u32 :: try_from (multis . len ()) . unwrap () - 1) } ; mappings . push ((CharEscape (key) , value)) ; } let mut tables = String :: new () ; let mut size = 0 ; size += size_of_val (mappings . as_slice ()) ; write ! (tables , "static {}CASE_TABLE: &[(char, u32); {}] = &[{}];" , case , mappings . len () , fmt_list (mappings) ,) . unwrap () ; tables . push_str ("\n\n") ; size += size_of_val (multis . as_slice ()) ; write ! (tables , "static {}CASE_TABLE_MULTI: &[[char; 3]; {}] = &[{}];" , case , multis . len () , fmt_list (multis) ,) . unwrap () ; (tables , size) }
};
}
