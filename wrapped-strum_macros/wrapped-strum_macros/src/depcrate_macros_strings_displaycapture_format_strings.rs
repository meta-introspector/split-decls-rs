// Generated macro for capture_format_strings (function)
macro_rules! Depcrate_macros_strings_displaycapture_format_strings {
() => {
// Module: crate::macros::strings::display
// Provides: {"capture_format_strings"}
// Dependencies: {}
fn capture_format_strings (string_literal : & LitStr) -> syn :: Result < Vec < String > > { let format_str = string_literal . value () . replace ("{{" , "") . replace ("}}" , "") ; let mut new_var_start_index : Option < usize > = None ; let mut var_used = Vec :: new () ; for (i , chr) in format_str . bytes () . enumerate () { if chr == b'{' { if new_var_start_index . is_some () { return Err (syn :: Error :: new_spanned (string_literal , "Bracket opened without closing previous bracket" ,)) ; } new_var_start_index = Some (i) ; continue ; } if chr == b'}' { let start_index = new_var_start_index . take () . ok_or (syn :: Error :: new_spanned (string_literal , "Bracket closed without previous opened bracket" ,)) ? ; let inside_brackets = & format_str [start_index + 1 .. i] ; let ident_str = inside_brackets . split (":") . next () . unwrap () . trim_end () ; var_used . push (ident_str . to_owned ()) ; } } Ok (var_used) }
};
}
