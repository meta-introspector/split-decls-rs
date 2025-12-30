// Generated macro for SnippetProvider (struct)
macro_rules! Depcrate_modules_visitorSnippetProvider {
() => {
// Module: crate::modules::visitor
// Provides: {"SnippetProvider"}
// Dependencies: {}
# [doc = " Creates a string slice corresponding to the specified span."] pub (crate) struct SnippetProvider { # [doc = " A pointer to the content of the file we are formatting."] big_snippet : Arc < String > , # [doc = " A position of the start of `big_snippet`, used as an offset."] start_pos : usize , # [doc = " An end position of the file that this snippet lives."] end_pos : usize , }
};
}
