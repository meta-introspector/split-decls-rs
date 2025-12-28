use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Associative operator."] # [derive (Copy , Clone , PartialEq , Debug)] pub enum AssocOp { # [doc = " A binary op."] Binary (BinOpKind) , # [doc = " `?=` where ? is one of the assignable BinOps"] AssignOp (AssignOpKind) , # [doc = " `=`"] Assign , # [doc = " `as`"] Cast , # [doc = " `..` or `..=` range"] Range (RangeLimits) , }
}