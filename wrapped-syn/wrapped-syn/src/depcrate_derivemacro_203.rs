// Generated macro for macro_203 (macro)
macro_rules! Depcrate_derivemacro_203 {
() => {
// Module: crate::derive
// Provides: {"macro_203"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " Body of a derived struct or enum."] pub enum Body { # [doc = " It's an enum."] pub Enum (BodyEnum { pub enum_token : tokens :: Enum , pub brace_token : tokens :: Brace , pub variants : Delimited < Variant , tokens :: Comma >, }) , # [doc = " It's a struct."] pub Struct (BodyStruct { pub data : VariantData , pub struct_token : tokens :: Struct , pub semi_token : Option < tokens :: Semi >, }) , } do_not_generate_to_tokens }
};
}
