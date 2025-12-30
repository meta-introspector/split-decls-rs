// Generated macro for RecursiveTransform (struct)
macro_rules! Depcrate_transformRecursiveTransform {
() => {
// Module: crate::transform
// Provides: {"RecursiveTransform"}
// Dependencies: {}
# [doc = " A helper struct that can wrap a non-recursive [`Transform`] (i.e. one that does not apply to"] # [doc = " subschemas) into a recursive one."] # [doc = ""] # [doc = " Its implementation of `Transform` will first apply the inner transform to the \"parent\" schema,"] # [doc = " and then its subschemas (and their subschemas, and so on)."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use schemars::{Schema, json_schema};"] # [doc = " use schemars::transform::{Transform, RecursiveTransform};"] # [doc = ""] # [doc = " let mut transform = RecursiveTransform(|schema: &mut Schema| {"] # [doc = "     schema.insert(\"my_property\".to_string(), \"hello world\".into());"] # [doc = " });"] # [doc = ""] # [doc = " let mut schema = json_schema!({"] # [doc = "     \"type\": \"array\","] # [doc = "     \"items\": {}"] # [doc = " });"] # [doc = ""] # [doc = " transform.transform(&mut schema);"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     schema,"] # [doc = "     json_schema!({"] # [doc = "         \"type\": \"array\","] # [doc = "         \"items\": {"] # [doc = "             \"my_property\": \"hello world\""] # [doc = "         },"] # [doc = "         \"my_property\": \"hello world\""] # [doc = "     })"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Clone)] # [allow (clippy :: exhaustive_structs)] pub struct RecursiveTransform < T > (pub T) ;
};
}
