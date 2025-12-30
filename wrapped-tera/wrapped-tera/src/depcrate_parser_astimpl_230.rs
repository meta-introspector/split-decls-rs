// Generated macro for impl_230 (impl)
macro_rules! Depcrate_parser_astimpl_230 {
() => {
// Module: crate::parser::ast
// Provides: {"impl_230"}
// Dependencies: {}
impl fmt :: Display for LogicOperator { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , match * self { LogicOperator :: Gt => ">" , LogicOperator :: Gte => ">=" , LogicOperator :: Lt => "<" , LogicOperator :: Lte => "<=" , LogicOperator :: Eq => "==" , LogicOperator :: NotEq => "!=" , LogicOperator :: And => "and" , LogicOperator :: Or => "or" , }) } }
};
}
