// Generated macro for tests (module)
macro_rules! Depcrate_parser_functiontests {
() => {
// Module: crate::parser::function
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: parser :: tests ; # [test] fn test_bats_test () { assert_parse ! (bats_test ("@test 'can parse' {\n  true\n}") , Function :: new ("'can parse'" , tests :: pipeline ("true"))) ; assert_parse ! (bats_test ("@test random text !(@*$Y&! {\n  true\n}") , Function :: new ("random text !(@*$Y&!" , tests :: pipeline ("true"))) ; assert_parse ! (bats_test ("@test foo { bar { baz {\n  true\n}") , Function :: new ("foo { bar { baz" , tests :: pipeline ("true"))) ; assert_parse ! (bats_test ("@test foo \n{\n true\n}") => Err ((1 , 7) , Notes : [((1 , 7) , "invalid test name")])) ; } # [test] fn test_function () { assert_parse ! (function (":() { foo; }") , Function :: new (":" , tests :: pipeline ("foo"))) ; assert_parse ! (function ("function []!() { foo; }") , Function :: new ("[]!" , tests :: pipeline ("foo"))) ; assert_parse ! (function ("foo() (bar)") , Function :: new ("foo" , tests :: pipeline ("bar"))) ; assert_parse ! (function ("foo(x, y) { bar; }") , Function :: new ("foo" , tests :: pipeline ("bar")) , [((1 , 5) , (1 , 9) , ParseDiagnosticKind :: NotShellCode)]) ; assert_parse ! (function ("function foo{ bar; }") , Function :: new ("foo" , tests :: pipeline ("bar")) , [((1 , 13) , ParseDiagnosticKind :: MissingSpace)]) ; assert_parse ! (function ("function foo\n  bar;") => Err ((2 , 3) , Notes : [((2 , 3) , "expected a '{' or '('")])) ; assert_parse ! (function ("function\nfoo() { bar; }") => Err ((1 , 9) , Notes : [((1 , 1) , "invalid function signature")])) ; } }
};
}
