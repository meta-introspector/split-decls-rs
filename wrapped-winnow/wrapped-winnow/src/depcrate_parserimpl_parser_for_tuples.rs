// Generated macro for impl_parser_for_tuples (macro)
macro_rules! Depcrate_parserimpl_parser_for_tuples {
() => {
// Module: crate::parser
// Provides: {"impl_parser_for_tuples"}
// Dependencies: {}
macro_rules ! impl_parser_for_tuples { ($ index1 : tt $ parser1 : ident $ output1 : ident , $ ($ index : tt $ parser : ident $ output : ident) ,+) => { impl_parser_for_tuples ! (__impl $ index1 $ parser1 $ output1 ; $ ($ index $ parser $ output) ,+) ; } ; (__impl $ ($ index : tt $ parser : ident $ output : ident) ,+; $ index1 : tt $ parser1 : ident $ output1 : ident $ (,$ index2 : tt $ parser2 : ident $ output2 : ident) *) => { impl_parser_for_tuple ! ($ ($ index $ parser $ output) ,+) ; impl_parser_for_tuples ! (__impl $ ($ index $ parser $ output) ,+, $ index1 $ parser1 $ output1 ; $ ($ index2 $ parser2 $ output2) ,*) ; } ; (__impl $ ($ index : tt $ parser : ident $ output : ident) ,+;) => { impl_parser_for_tuple ! ($ ($ index $ parser $ output) ,+) ; } }
};
}
