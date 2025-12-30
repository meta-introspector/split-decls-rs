// Generated macro for Codec (trait)
macro_rules! Depcrate_msgs_codecCodec {
() => {
// Module: crate::msgs::codec
// Provides: {"Codec"}
// Dependencies: {}
# [doc = " Trait for implementing encoding and decoding functionality"] # [doc = " on something."] pub trait Codec < 'a > : Debug + Sized { # [doc = " Function for encoding itself by appending itself to"] # [doc = " the provided vec of bytes."] fn encode (& self , bytes : & mut Vec < u8 >) ; # [doc = " Function for decoding itself from the provided reader"] # [doc = " will return Some if the decoding was successful or"] # [doc = " None if it was not."] fn read (_ : & mut Reader < 'a >) -> Result < Self , InvalidMessage > ; # [doc = " Convenience function for encoding the implementation"] # [doc = " into a vec and returning it"] fn get_encoding (& self) -> Vec < u8 > { let mut bytes = Vec :: new () ; self . encode (& mut bytes) ; bytes } # [doc = " Function for wrapping a call to the read function in"] # [doc = " a Reader for the slice of bytes provided"] # [doc = ""] # [doc = " Returns `Err(InvalidMessage::ExcessData(_))` if"] # [doc = " `Self::read` does not read the entirety of `bytes`."] fn read_bytes (bytes : & 'a [u8]) -> Result < Self , InvalidMessage > { let mut reader = Reader :: init (bytes) ; Self :: read (& mut reader) . and_then (| r | { reader . expect_empty ("read_bytes") ? ; Ok (r) }) } }
};
}
