// Generated macro for impl_114 (impl)
macro_rules! Depcrate_errorimpl_114 {
() => {
// Module: crate::error
// Provides: {"impl_114"}
// Dependencies: {}
impl core :: fmt :: Display for StrContextValue { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: CharLiteral ('\n') => "newline" . fmt (f) , Self :: CharLiteral ('`') => "'`'" . fmt (f) , Self :: CharLiteral (c) if c . is_ascii_control () => { write ! (f , "`{}`" , c . escape_debug ()) } Self :: CharLiteral (c) => write ! (f , "`{c}`") , Self :: StringLiteral (c) => write ! (f , "`{c}`") , Self :: Description (c) => write ! (f , "{c}") , } } }
};
}
