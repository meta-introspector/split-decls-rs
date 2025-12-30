// Generated macro for impl_99 (impl)
macro_rules! Depcrate_internallyimpl_99 {
() => {
// Module: crate::internally
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for KeyVisitor { type Value = Key ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a key in dyn {}" , self . trait_object) } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { if value == self . tag { Ok (Key :: Tag) } else { Ok (Key :: Other (value . to_owned ())) } } }
};
}
