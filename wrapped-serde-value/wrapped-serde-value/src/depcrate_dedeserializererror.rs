// Generated macro for DeserializerError (enum)
macro_rules! Depcrate_deDeserializerError {
() => {
// Module: crate::de
// Provides: {"DeserializerError"}
// Dependencies: {}
# [derive (Debug)] pub enum DeserializerError { Custom (String) , InvalidType (Unexpected , String) , InvalidValue (Unexpected , String) , InvalidLength (usize , String) , UnknownVariant (String , & 'static [& 'static str]) , UnknownField (String , & 'static [& 'static str]) , MissingField (& 'static str) , DuplicateField (& 'static str) , }
};
}
