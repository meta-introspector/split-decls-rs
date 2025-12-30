// Generated macro for impl_228 (impl)
macro_rules! Depcrate_parser_astimpl_228 {
() => {
// Module: crate::parser::ast
// Provides: {"impl_228"}
// Dependencies: {}
impl fmt :: Display for MathOperator { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , match * self { MathOperator :: Add => "+" , MathOperator :: Sub => "-" , MathOperator :: Mul => "*" , MathOperator :: Div => "/" , MathOperator :: Modulo => "%" , }) } }
};
}
