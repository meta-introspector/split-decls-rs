// Generated macro for impl_78 (impl)
macro_rules! Depcrate_expressionimpl_78 {
() => {
// Module: crate::expression
// Provides: {"impl_78"}
// Dependencies: {}
impl fmt :: Display for Expression { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Identifier (identifier , kind) => { write ! (f , "{}{identifier}" , matches ! (kind , IdentifierType :: Variable) . then_some ("$") . unwrap_or_default ()) } Self :: MacroCall (name , expression) => { write ! (f , "{name}!({expression})") } _ => Err (fmt :: Error) , } } }
};
}
