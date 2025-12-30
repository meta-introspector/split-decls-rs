// Generated macro for impl_14 (impl)
macro_rules! Depcrate_helpers_case_styleimpl_14 {
() => {
// Module: crate::helpers::case_style
// Provides: {"impl_14"}
// Dependencies: {}
impl FromStr for CaseStyle { type Err = () ; fn from_str (text : & str) -> Result < Self , () > { Ok (match text { "PascalCase" | "camel_case" => CaseStyle :: PascalCase , "camelCase" => CaseStyle :: CamelCase , "snake_case" | "snek_case" => CaseStyle :: SnakeCase , "kebab-case" | "kebab_case" => CaseStyle :: KebabCase , "SCREAMING-KEBAB-CASE" => CaseStyle :: ScreamingKebabCase , "SCREAMING_SNAKE_CASE" | "shouty_snake_case" | "shouty_snek_case" => { CaseStyle :: ShoutySnakeCase } "title_case" => CaseStyle :: TitleCase , "mixed_case" => CaseStyle :: MixedCase , "lowercase" => CaseStyle :: LowerCase , "UPPERCASE" => CaseStyle :: UpperCase , "Train-Case" => CaseStyle :: TrainCase , _ => return Err (()) , }) } }
};
}
