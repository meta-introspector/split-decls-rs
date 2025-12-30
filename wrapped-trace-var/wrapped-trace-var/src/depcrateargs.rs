// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [doc = " Parses a list of variable names separated by commas."] # [doc = ""] # [doc = "     a, b, c"] # [doc = ""] # [doc = " This is how the compiler passes in arguments to our attribute -- it is"] # [doc = " everything inside the delimiters after the attribute name."] # [doc = ""] # [doc = "     #[trace_var(a, b, c)]"] # [doc = "                 ^^^^^^^"] struct Args { vars : Set < Ident > , }
};
}
