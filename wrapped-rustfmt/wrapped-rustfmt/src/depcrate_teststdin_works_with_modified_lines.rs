// Generated macro for stdin_works_with_modified_lines (function)
macro_rules! Depcrate_teststdin_works_with_modified_lines {
() => {
// Module: crate::test
// Provides: {"stdin_works_with_modified_lines"}
// Dependencies: {}
# [doc = " Ensures that `EmitMode::ModifiedLines` works with input from `stdin`. Useful"] # [doc = " when embedding Rustfmt (e.g. inside RLS)."] # [test] fn stdin_works_with_modified_lines () { init_log () ; let input = "\nfn\n some( )\n{\n}\nfn main () {}\n" ; let output = "1 6 2\nfn some() {}\nfn main() {}\n" ; let input = Input :: Text (input . to_owned ()) ; let mut config = Config :: default () ; config . set () . newline_style (NewlineStyle :: Unix) ; config . set () . emit_mode (EmitMode :: ModifiedLines) ; let mut buf : Vec < u8 > = vec ! [] ; { let mut session = Session :: new (config , Some (& mut buf)) ; session . format (input) . unwrap () ; let errors = ReportedErrors { has_diff : true , .. Default :: default () } ; assert_eq ! (session . errors , errors) ; } assert_eq ! (buf , output . as_bytes ()) ; }
};
}
