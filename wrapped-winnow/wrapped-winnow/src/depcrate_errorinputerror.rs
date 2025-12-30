// Generated macro for InputError (struct)
macro_rules! Depcrate_errorInputError {
() => {
// Module: crate::error
// Provides: {"InputError"}
// Dependencies: {}
# [doc = " Capture input on error"] # [doc = ""] # [doc = " This is useful for testing of generic parsers to ensure the error happens at the right"] # [doc = " location."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** [context][Parser::context] and inner errors (like from [`Parser::try_map`]) will be"] # [doc = " dropped."] # [doc = ""] # [doc = " </div>"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct InputError < I : Clone > { # [doc = " The input stream, pointing to the location where the error occurred"] pub input : I , }
};
}
