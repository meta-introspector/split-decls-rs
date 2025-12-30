// Generated macro for impl_parse_tuple (macro)
macro_rules! Depcrateimpl_parse_tuple {
() => {
// Module: crate
// Provides: {"impl_parse_tuple"}
// Dependencies: {}
macro_rules ! impl_parse_tuple { ($ ty : ty) => { impl ParseTuple for ($ ty ,) { fn parse (input : & [& str]) -> Self { assert_eq ! (input . len () , 1 , "expected a single argument, got {input:?}") ; (parse (input , 0) ,) } } impl ParseTuple for ($ ty , $ ty) { fn parse (input : & [& str]) -> Self { assert_eq ! (input . len () , 2 , "expected two arguments, got {input:?}") ; (parse (input , 0) , parse (input , 1)) } } impl ParseTuple for ($ ty , i32) { fn parse (input : & [& str]) -> Self { assert_eq ! (input . len () , 2 , "expected two arguments, got {input:?}") ; (parse (input , 0) , parse (input , 1)) } } impl ParseTuple for (i32 , $ ty) { fn parse (input : & [& str]) -> Self { assert_eq ! (input . len () , 2 , "expected two arguments, got {input:?}") ; (parse (input , 0) , parse (input , 1)) } } impl ParseTuple for ($ ty , $ ty , $ ty) { fn parse (input : & [& str]) -> Self { assert_eq ! (input . len () , 3 , "expected three arguments, got {input:?}") ; (parse (input , 0) , parse (input , 1) , parse (input , 2)) } } } ; }
};
}
