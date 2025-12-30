// Generated macro for serialize (function)
macro_rules! Depcrate_internallyserialize {
() => {
// Module: crate::internally
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < S , T > (serializer : S , tag : & 'static str , variant : & 'static str , concrete : & T ,) -> Result < S :: Ok , S :: Error > where S : Serializer , T : ? Sized + erased_serde :: Serialize , { let adapter = InternallyTaggedSerializer { tag , variant , delegate : serializer , } ; Wrap (concrete) . serialize (adapter) }
};
}
