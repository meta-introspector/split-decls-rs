use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A segment of a path: an identifier, an optional lifetime, and a set of types."] # [doc = ""] # [doc = " E.g., `std`, `String` or `Box<T>`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct PathSegment { # [doc = " The identifier portion of this path segment."] pub ident : Ident , pub id : NodeId , # [doc = " Type/lifetime parameters attached to this path. They come in"] # [doc = " two flavors: `Path<A,B,C>` and `Path(A,B) -> C`."] # [doc = " `None` means that no parameter list is supplied (`Path`),"] # [doc = " `Some` means that parameter list is supplied (`Path<X, Y>`)"] # [doc = " but it can be empty (`Path<>`)."] # [doc = " `P` is used as a size optimization for the common case with no parameters."] pub args : Option < Box < GenericArgs > > , }
}