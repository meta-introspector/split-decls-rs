// Generated macro for VarInt (trait)
macro_rules! DepcrateVarInt {
() => {
// Module: crate
// Provides: {"VarInt"}
// Dependencies: {}
pub trait VarInt : Sized { fn visit_seq < 'de , A > (seq : A) -> Result < Self , A :: Error > where A : SeqAccess < 'de > ; fn serialize < S > (self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer ; }
};
}
