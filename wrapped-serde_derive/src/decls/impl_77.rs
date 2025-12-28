macro_rules! deps {
    () => {
        ParseError!();
        RenameRule!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl RenameRule { pub fn from_str (rename_all_str : & str) -> Result < Self , ParseError > { for (name , rule) in RENAME_RULES { if rename_all_str == * name { return Ok (* rule) ; } } Err (ParseError { unknown : rename_all_str , }) } # [doc = " Apply a renaming rule to an enum variant, returning the version expected in the source."] pub fn apply_to_variant (self , variant : & str) -> String { match self { None | PascalCase => variant . to_owned () , LowerCase => variant . to_ascii_lowercase () , UpperCase => variant . to_ascii_uppercase () , CamelCase => variant [.. 1] . to_ascii_lowercase () + & variant [1 ..] , SnakeCase => { let mut snake = String :: new () ; for (i , ch) in variant . char_indices () { if i > 0 && ch . is_uppercase () { snake . push ('_') ; } snake . push (ch . to_ascii_lowercase ()) ; } snake } ScreamingSnakeCase => SnakeCase . apply_to_variant (variant) . to_ascii_uppercase () , KebabCase => SnakeCase . apply_to_variant (variant) . replace ('_' , "-") , ScreamingKebabCase => ScreamingSnakeCase . apply_to_variant (variant) . replace ('_' , "-") , } } # [doc = " Apply a renaming rule to a struct field, returning the version expected in the source."] pub fn apply_to_field (self , field : & str) -> String { match self { None | LowerCase | SnakeCase => field . to_owned () , UpperCase => field . to_ascii_uppercase () , PascalCase => { let mut pascal = String :: new () ; let mut capitalize = true ; for ch in field . chars () { if ch == '_' { capitalize = true ; } else if capitalize { pascal . push (ch . to_ascii_uppercase ()) ; capitalize = false ; } else { pascal . push (ch) ; } } pascal } CamelCase => { let pascal = PascalCase . apply_to_field (field) ; pascal [.. 1] . to_ascii_lowercase () + & pascal [1 ..] } ScreamingSnakeCase => field . to_ascii_uppercase () , KebabCase => field . replace ('_' , "-") , ScreamingKebabCase => ScreamingSnakeCase . apply_to_field (field) . replace ('_' , "-") , } } # [doc = " Returns the `RenameRule` if it is not `None`, `rule_b` otherwise."] pub fn or (self , rule_b : Self) -> Self { match self { None => rule_b , _ => self , } } }
    };
}

impl_77!()