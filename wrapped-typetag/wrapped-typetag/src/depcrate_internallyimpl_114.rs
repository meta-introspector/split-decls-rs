// Generated macro for impl_114 (impl)
macro_rules! Depcrate_internallyimpl_114 {
() => {
// Module: crate::internally
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for DefaultKey { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "the string \"{}\"" , DEFAULT_KEY) } fn visit_str < E > (self , string : & str) -> Result < Self :: Value , E > where E : de :: Error , { if string == DEFAULT_KEY { Ok (()) } else { Err (de :: Error :: unknown_field (string , & [DEFAULT_KEY])) } } }
};
}
