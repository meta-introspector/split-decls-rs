// Generated macro for serialize (function)
macro_rules! Depcrate_externallyserialize {
() => {
// Module: crate::externally
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < S , T > (serializer : S , variant : & 'static str , concrete : & T ,) -> Result < S :: Ok , S :: Error > where S : Serializer , T : ? Sized + erased_serde :: Serialize , { let mut ser = serializer . serialize_map (Some (1)) ? ; ser . serialize_entry (variant , & Wrap (concrete)) ? ; ser . end () }
};
}
