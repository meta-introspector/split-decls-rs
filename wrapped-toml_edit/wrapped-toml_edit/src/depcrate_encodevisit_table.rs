// Generated macro for visit_table (function)
macro_rules! Depcrate_encodevisit_table {
() => {
// Module: crate::encode
// Provides: {"visit_table"}
// Dependencies: {}
fn visit_table (mut buf : & mut dyn Write , input : Option < & str > , table : & Table , path : & [Key] , is_array_of_tables : bool , first_table : & mut bool ,) -> Result { let children = table . get_values () ; let is_visible_std_table = ! (table . implicit && children . is_empty ()) ; if path . is_empty () { if ! children . is_empty () { * first_table = false ; } } else if is_array_of_tables { let default_decor = if * first_table { * first_table = false ; ("" , DEFAULT_TABLE_DECOR . 1) } else { DEFAULT_TABLE_DECOR } ; table . decor . prefix_encode (buf , input , default_decor . 0) ? ; buf . open_array_of_tables_header () ? ; encode_key_path (path , buf , input , DEFAULT_KEY_PATH_DECOR) ? ; buf . close_array_of_tables_header () ? ; table . decor . suffix_encode (buf , input , default_decor . 1) ? ; writeln ! (buf) ? ; } else if is_visible_std_table { let default_decor = if * first_table { * first_table = false ; ("" , DEFAULT_TABLE_DECOR . 1) } else { DEFAULT_TABLE_DECOR } ; table . decor . prefix_encode (buf , input , default_decor . 0) ? ; buf . open_table_header () ? ; encode_key_path (path , buf , input , DEFAULT_KEY_PATH_DECOR) ? ; buf . close_table_header () ? ; table . decor . suffix_encode (buf , input , default_decor . 1) ? ; writeln ! (buf) ? ; } for (key_path , value) in children { encode_key_path_ref (& key_path , buf , input , DEFAULT_KEY_DECOR) ? ; buf . keyval_sep () ? ; encode_value (value , buf , input , DEFAULT_VALUE_DECOR) ? ; writeln ! (buf) ? ; } Ok (()) }
};
}
