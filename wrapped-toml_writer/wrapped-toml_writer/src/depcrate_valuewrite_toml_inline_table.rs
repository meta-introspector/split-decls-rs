// Generated macro for write_toml_inline_table (function)
macro_rules! Depcrate_valuewrite_toml_inline_table {
() => {
// Module: crate::value
// Provides: {"write_toml_inline_table"}
// Dependencies: {}
fn write_toml_inline_table < 'i , I : Iterator < Item = (& 'i K , & 'i V) > , K : WriteTomlKey + 'i , V : WriteTomlValue + 'i , W : TomlWrite + ? Sized , > (mut iter : I , writer : & mut W ,) -> core :: fmt :: Result { writer . open_inline_table () ? ; let mut trailing_space = false ; if let Some ((key , value)) = iter . next () { writer . space () ? ; writer . key (key) ? ; writer . space () ? ; writer . keyval_sep () ? ; writer . space () ? ; writer . value (value) ? ; trailing_space = true ; } for (key , value) in iter { writer . val_sep () ? ; writer . space () ? ; writer . key (key) ? ; writer . space () ? ; writer . keyval_sep () ? ; writer . space () ? ; writer . value (value) ? ; } if trailing_space { writer . space () ? ; } writer . close_inline_table () ? ; Ok (()) }
};
}
