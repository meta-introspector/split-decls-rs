// Generated macro for EnumMessage (trait)
macro_rules! DepcrateEnumMessage {
() => {
// Module: crate
// Provides: {"EnumMessage"}
// Dependencies: {}
# [doc = " Associates additional pieces of information with an Enum. This can be"] # [doc = " autoimplemented by deriving `EnumMessage` and annotating your variants with"] # [doc = " `#[strum(message=\"...\")]`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::fmt::Debug;"] # [doc = " // You need to bring the type into scope to use it!!!"] # [doc = " use strum::EnumMessage;"] # [doc = ""] # [doc = " #[derive(PartialEq, Eq, Debug, EnumMessage)]"] # [doc = " enum Pet {"] # [doc = "     #[strum(message=\"I have a dog\")]"] # [doc = "     #[strum(detailed_message=\"My dog's name is Spots\")]"] # [doc = "     Dog,"] # [doc = "     /// I am documented."] # [doc = "     #[strum(message=\"I don't have a cat\")]"] # [doc = "     Cat,"] # [doc = " }"] # [doc = ""] # [doc = " let my_pet = Pet::Dog;"] # [doc = " assert_eq!(\"I have a dog\", my_pet.get_message().unwrap());"] # [doc = " ```"] pub trait EnumMessage { fn get_message (& self) -> Option < & 'static str > ; fn get_detailed_message (& self) -> Option < & 'static str > ; # [doc = " Get the doc comment associated with a variant if it exists."] fn get_documentation (& self) -> Option < & 'static str > ; fn get_serializations (& self) -> & 'static [& 'static str] ; }
};
}
