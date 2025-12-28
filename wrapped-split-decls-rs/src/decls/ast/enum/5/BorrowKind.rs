use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The kind of borrow in an `AddrOf` expression,"] # [doc = " e.g., `&place` or `&raw const place`."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub enum BorrowKind { # [doc = " A normal borrow, `&$expr` or `&mut $expr`."] # [doc = " The resulting type is either `&'a T` or `&'a mut T`"] # [doc = " where `T = typeof($expr)` and `'a` is some lifetime."] Ref , # [doc = " A raw borrow, `&raw const $expr` or `&raw mut $expr`."] # [doc = " The resulting type is either `*const T` or `*mut T`"] # [doc = " where `T = typeof($expr)`."] Raw , # [doc = " A pinned borrow, `&pin const $expr` or `&pin mut $expr`."] # [doc = " The resulting type is either `Pin<&'a T>` or `Pin<&'a mut T>`"] # [doc = " where `T = typeof($expr)` and `'a` is some lifetime."] Pin , }
}