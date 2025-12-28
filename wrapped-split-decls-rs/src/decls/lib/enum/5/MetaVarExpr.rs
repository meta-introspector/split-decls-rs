use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " A meta-variable expression, for expansions based on properties of meta-variables."] # [derive (Debug , PartialEq , Encodable , Decodable)] pub enum MetaVarExpr { # [doc = " Unification of two or more identifiers."] Concat (Box < [MetaVarExprConcatElem] >) , # [doc = " The number of repetitions of an identifier."] Count (Ident , usize) , # [doc = " Ignore a meta-variable for repetition without expansion."] Ignore (Ident) , # [doc = " The index of the repetition at a particular depth, where 0 is the innermost"] # [doc = " repetition. The `usize` is the depth."] Index (usize) , # [doc = " The length of the repetition at a particular depth, where 0 is the innermost"] # [doc = " repetition. The `usize` is the depth."] Len (usize) , }
}