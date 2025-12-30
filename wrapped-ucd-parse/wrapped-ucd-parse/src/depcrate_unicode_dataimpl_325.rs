// Generated macro for impl_325 (impl)
macro_rules! Depcrate_unicode_dataimpl_325 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_325"}
// Dependencies: {}
impl std :: str :: FromStr for UnicodeData { type Err = Error ; fn from_str (line : & str) -> Result < UnicodeData , Error > { let re_parts = regex ! (r"(?x)
                ^
                ([A-Z0-9]+);  #  1; codepoint
                ([^;]+);      #  2; name
                ([^;]+);      #  3; general category
                ([0-9]+);     #  4; canonical combining class
                ([^;]+);      #  5; bidi class
                ([^;]*);      #  6; decomposition
                ([0-9]*);     #  7; numeric type decimal
                ([0-9]*);     #  8; numeric type digit
                ([-0-9/]*);   #  9; numeric type numeric
                ([YN]);       # 10; bidi mirrored
                ([^;]*);      # 11; unicode1 name
                ([^;]*);      # 12; ISO comment
                ([^;]*);      # 13; simple uppercase mapping
                ([^;]*);      # 14; simple lowercase mapping
                ([^;]*)       # 15; simple titlecase mapping
                $
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid UnicodeData line") , } ; let capget = | n | caps . get (n) . unwrap () . as_str () ; let mut data = UnicodeData :: default () ; data . codepoint = capget (1) . parse () ? ; data . name = capget (2) . to_string () ; data . general_category = capget (3) . to_string () ; data . canonical_combining_class = match capget (4) . parse () { Ok (n) => n , Err (err) => { return err ! ("failed to parse canonical combining class '{}': {}" , capget (4) , err) } } ; data . bidi_class = capget (5) . to_string () ; if ! caps [6] . is_empty () { data . decomposition = caps [6] . parse () ? ; } else { data . decomposition . push (data . codepoint) ? ; } if ! capget (7) . is_empty () { data . numeric_type_decimal = Some (match capget (7) . parse () { Ok (n) => n , Err (err) => { return err ! ("failed to parse numeric type decimal '{}': {}" , capget (7) , err) } }) ; } if ! capget (8) . is_empty () { data . numeric_type_digit = Some (match capget (8) . parse () { Ok (n) => n , Err (err) => { return err ! ("failed to parse numeric type digit '{}': {}" , capget (8) , err) } }) ; } if ! capget (9) . is_empty () { data . numeric_type_numeric = Some (capget (9) . parse () ?) ; } data . bidi_mirrored = capget (10) == "Y" ; data . unicode1_name = capget (11) . to_string () ; data . iso_comment = capget (12) . to_string () ; if ! capget (13) . is_empty () { data . simple_uppercase_mapping = Some (capget (13) . parse () ?) ; } if ! capget (14) . is_empty () { data . simple_lowercase_mapping = Some (capget (14) . parse () ?) ; } if ! capget (15) . is_empty () { data . simple_titlecase_mapping = Some (capget (15) . parse () ?) ; } Ok (data) } }
};
}
