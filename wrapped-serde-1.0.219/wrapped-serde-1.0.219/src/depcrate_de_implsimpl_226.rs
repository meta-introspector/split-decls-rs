// Generated macro for impl_226 (impl)
macro_rules! Depcrate_de_implsimpl_226 {
() => {
// Module: crate::de::impls
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for ArrayVisitor < [T ; 0] > { type Value = [T ; 0] ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("an empty array") } # [inline] fn visit_seq < A > (self , _ : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { Ok ([]) } }
};
}
