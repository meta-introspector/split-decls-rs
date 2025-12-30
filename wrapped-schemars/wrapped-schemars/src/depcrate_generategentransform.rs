// Generated macro for GenTransform (trait)
macro_rules! Depcrate_generateGenTransform {
() => {
// Module: crate::generate
// Provides: {"GenTransform"}
// Dependencies: {}
# [doc = " A [`Transform`] which implements additional traits required to be included in a"] # [doc = " [`SchemaSettings`]."] # [doc = ""] # [doc = " You will rarely need to use this trait directly as it is automatically implemented for any type"] # [doc = " which implements all of:"] # [doc = " - [`Transform`]"] # [doc = " - [`std::any::Any`] (implemented for all `'static` types)"] # [doc = " - [`std::clone::Clone`]"] # [doc = " - [`std::marker::Send`]"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use schemars::transform::Transform;"] # [doc = " use schemars::generate::GenTransform;"] # [doc = ""] # [doc = " #[derive(Debug, Clone)]"] # [doc = " struct MyTransform;"] # [doc = ""] # [doc = " impl Transform for MyTransform {"] # [doc = "   fn transform(&mut self, schema: &mut schemars::Schema) {"] # [doc = "     todo!()"] # [doc = "   }"] # [doc = " }"] # [doc = ""] # [doc = " let v: &dyn GenTransform = &MyTransform;"] # [doc = " assert!(v.is::<MyTransform>());"] # [doc = " ```"] pub trait GenTransform : Transform + DynClone + Any + Send { # [deprecated = "Only to support pre-1.86 rustc"] # [doc (hidden)] fn _as_any (& self) -> & dyn Any ; # [deprecated = "Only to support pre-1.86 rustc"] # [doc (hidden)] fn _as_any_mut (& mut self) -> & mut dyn Any ; # [deprecated = "Only to support pre-1.86 rustc"] # [doc (hidden)] fn _into_any (self : Box < Self >) -> Box < dyn Any > ; }
};
}
