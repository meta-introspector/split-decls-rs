// Generated macro for parse_smoke_test (function)
macro_rules! Depcrate_testsparse_smoke_test {
() => {
// Module: crate::tests
// Provides: {"parse_smoke_test"}
// Dependencies: {}
# [test] fn parse_smoke_test () { let code = r#"
fn main() {
    println!("Hello, world!")
}
    "# ; let parse = SourceFile :: parse (code , Edition :: CURRENT) ; assert ! (parse . ok () . is_ok ()) ; }
};
}
