use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S > SerdeMapVisitor < S > where S : SerializeMap , { # [doc = " Create a new map visitor."] pub fn new (serializer : S) -> Self { Self { serializer , state : Ok (()) , } } # [doc = " Completes serializing the visited object, returning `Ok(())` if all"] # [doc = " fields were serialized correctly, or `Error(S::Error)` if a field could"] # [doc = " not be serialized."] pub fn finish (self) -> Result < S :: Ok , S :: Error > { self . state ? ; self . serializer . end () } # [doc = " Completes serializing the visited object, returning ownership of the underlying serializer"] # [doc = " if all fields were serialized correctly, or `Err(S::Error)` if a field could not be"] # [doc = " serialized."] pub fn take_serializer (self) -> Result < S , S :: Error > { self . state ? ; Ok (self . serializer) } }
}