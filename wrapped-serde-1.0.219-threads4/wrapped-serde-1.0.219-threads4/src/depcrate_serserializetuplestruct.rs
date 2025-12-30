// Generated macro for SerializeTupleStruct (trait)
macro_rules! Depcrate_serSerializeTupleStruct {
() => {
// Module: crate::ser
// Provides: {"SerializeTupleStruct"}
// Dependencies: {}
# [doc = " Returned from `Serializer::serialize_tuple_struct`."] # [doc = ""] # [doc = " # Example use"] # [doc = ""] # [doc = " ```edition2021"] # [doc = " use serde::ser::{Serialize, SerializeTupleStruct, Serializer};"] # [doc = ""] # [doc = " struct Rgb(u8, u8, u8);"] # [doc = ""] # [doc = " impl Serialize for Rgb {"] # [doc = "     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>"] # [doc = "     where"] # [doc = "         S: Serializer,"] # [doc = "     {"] # [doc = "         let mut ts = serializer.serialize_tuple_struct(\"Rgb\", 3)?;"] # [doc = "         ts.serialize_field(&self.0)?;"] # [doc = "         ts.serialize_field(&self.1)?;"] # [doc = "         ts.serialize_field(&self.2)?;"] # [doc = "         ts.end()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example implementation"] # [doc = ""] # [doc = " The [example data format] presented on the website demonstrates an"] # [doc = " implementation of `SerializeTupleStruct` for a basic JSON data format."] # [doc = ""] # [doc = " [example data format]: https://serde.rs/data-format.html"] pub trait SerializeTupleStruct { # [doc = " Must match the `Ok` type of our `Serializer`."] type Ok ; # [doc = " Must match the `Error` type of our `Serializer`."] type Error : Error ; # [doc = " Serialize a tuple struct field."] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize ; # [doc = " Finish serializing a tuple struct."] fn end (self) -> Result < Self :: Ok , Self :: Error > ; }
};
}
