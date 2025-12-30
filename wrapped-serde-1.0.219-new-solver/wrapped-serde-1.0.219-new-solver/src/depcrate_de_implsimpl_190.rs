// Generated macro for impl_190 (impl)
macro_rules! Depcrate_de_implsimpl_190 {
() => {
// Module: crate::de::impls
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for CharVisitor { type Value = char ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a character") } # [inline] fn visit_char < E > (self , v : char) -> Result < Self :: Value , E > where E : Error , { Ok (v) } # [inline] fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { let mut iter = v . chars () ; match (iter . next () , iter . next ()) { (Some (c) , None) => Ok (c) , _ => Err (Error :: invalid_value (Unexpected :: Str (v) , & self)) , } } }
};
}
