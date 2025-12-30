// Generated macro for impl_16 (impl)
macro_rules! Depcrate_helpers_case_styleimpl_16 {
() => {
// Module: crate::helpers::case_style
// Provides: {"impl_16"}
// Dependencies: {}
impl CaseStyleHelpers for Ident { fn convert_case (& self , case_style : Option < CaseStyle >) -> String { let ident_string = self . to_string () ; if let Some (case_style) = case_style { match case_style { CaseStyle :: PascalCase => ident_string . to_upper_camel_case () , CaseStyle :: KebabCase => ident_string . to_kebab_case () , CaseStyle :: MixedCase => ident_string . to_lower_camel_case () , CaseStyle :: ShoutySnakeCase => ident_string . to_shouty_snake_case () , CaseStyle :: SnakeCase => ident_string . to_snake_case () , CaseStyle :: TitleCase => ident_string . to_title_case () , CaseStyle :: UpperCase => ident_string . to_uppercase () , CaseStyle :: LowerCase => ident_string . to_lowercase () , CaseStyle :: ScreamingKebabCase => ident_string . to_kebab_case () . to_uppercase () , CaseStyle :: TrainCase => ident_string . to_train_case () , CaseStyle :: CamelCase => { let camel_case = ident_string . to_upper_camel_case () ; let mut pascal = String :: with_capacity (camel_case . len ()) ; let mut it = camel_case . chars () ; if let Some (ch) = it . next () { pascal . extend (ch . to_lowercase ()) ; } pascal . extend (it) ; pascal } } } else { ident_string } } }
};
}
