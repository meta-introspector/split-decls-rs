// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for VarIntVisitor < T > where T : VarInt , { type Value = T ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a VarInt") } fn visit_seq < A > (self , seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { T :: visit_seq (seq) } }
};
}
