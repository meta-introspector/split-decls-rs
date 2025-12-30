// Generated macro for impl_40 (impl)
macro_rules! Depcrate_errorimpl_40 {
() => {
// Module: crate::error
// Provides: {"impl_40"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: MissingComponent { name , .. } => write ! (f , "missing component: {name}") , Self :: InvalidComponent { name , value , .. } => { write ! (f , "invalid component: {name} was {value}") } # [cfg (any (feature = "formatting" , feature = "parsing"))] Self :: ExpectedString { .. } => f . write_str ("expected string literal") , Self :: UnexpectedToken { tree } => write ! (f , "unexpected token: {tree}") , Self :: UnexpectedEndOfInput => f . write_str ("unexpected end of input") , Self :: Custom { message , .. } => f . write_str (message) , } } }
};
}
