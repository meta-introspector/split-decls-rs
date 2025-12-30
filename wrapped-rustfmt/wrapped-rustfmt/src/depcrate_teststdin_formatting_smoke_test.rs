// Generated macro for stdin_formatting_smoke_test (function)
macro_rules! Depcrate_teststdin_formatting_smoke_test {
() => {
// Module: crate::test
// Provides: {"stdin_formatting_smoke_test"}
// Dependencies: {}
# [test] fn stdin_formatting_smoke_test () { init_log () ; let input = Input :: Text ("fn main () {}" . to_owned ()) ; let mut config = Config :: default () ; config . set () . emit_mode (EmitMode :: Stdout) ; let mut buf : Vec < u8 > = vec ! [] ; { let mut session = Session :: new (config , Some (& mut buf)) ; session . format (input) . unwrap () ; assert ! (session . has_no_errors ()) ; } # [cfg (not (windows))] assert_eq ! (buf , "<stdin>:\n\nfn main() {}\n" . as_bytes ()) ; # [cfg (windows)] assert_eq ! (buf , "<stdin>:\n\nfn main() {}\r\n" . as_bytes ()) ; }
};
}
