// Generated macro for serialize (function)
macro_rules! Depcrate_adjacentlyserialize {
() => {
// Module: crate::adjacently
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < S , T > (serializer : S , trait_object : & 'static str , tag : & 'static str , variant : & 'static str , content : & 'static str , concrete : & T ,) -> Result < S :: Ok , S :: Error > where S : Serializer , T : ? Sized + erased_serde :: Serialize , { let mut ser = serializer . serialize_struct (trait_object , 2) ? ; ser . serialize_field (tag , variant) ? ; ser . serialize_field (content , & Wrap (concrete)) ? ; ser . end () }
};
}
