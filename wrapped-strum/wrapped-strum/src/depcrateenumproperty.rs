// Generated macro for EnumProperty (trait)
macro_rules! DepcrateEnumProperty {
() => {
// Module: crate
// Provides: {"EnumProperty"}
// Dependencies: {}
# [doc = " `EnumProperty` is a trait that makes it possible to store additional information"] # [doc = " with enum variants. This trait is designed to be used with the macro of the same"] # [doc = " name in the `strum_macros` crate. Currently, the string, integer and bool literals"] # [doc = " are supported in attributes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::fmt::Debug;"] # [doc = " // You need to bring the type into scope to use it!!!"] # [doc = " use strum::EnumProperty;"] # [doc = ""] # [doc = " #[derive(PartialEq, Eq, Debug, EnumProperty)]"] # [doc = " enum Class {"] # [doc = "     #[strum(props(Teacher=\"Ms.Frizzle\", Room=\"201\", students=16, mandatory=true))]"] # [doc = "     History,"] # [doc = "     #[strum(props(Teacher=\"Mr.Smith\"))]"] # [doc = "     #[strum(props(Room=\"103\", students=10))]"] # [doc = "     Mathematics,"] # [doc = "     #[strum(props(Time=\"2:30\", mandatory=true))]"] # [doc = "     Science,"] # [doc = " }"] # [doc = ""] # [doc = " let history = Class::History;"] # [doc = " assert_eq!(\"Ms.Frizzle\", history.get_str(\"Teacher\").unwrap());"] # [doc = " assert_eq!(16, history.get_int(\"students\").unwrap());"] # [doc = " assert!(history.get_bool(\"mandatory\").unwrap());"] # [doc = " ```"] pub trait EnumProperty { fn get_str (& self , prop : & str) -> Option < & 'static str > ; fn get_int (& self , _prop : & str) -> Option < i64 > ; fn get_bool (& self , _prop : & str) -> Option < bool > ; }
};
}
