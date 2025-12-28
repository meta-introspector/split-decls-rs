use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Controls whether the arguments are tupled. This is used for the call"] # [doc = " operator."] # [doc = ""] # [doc = " Tupling means that all call-side arguments are packed into a tuple and"] # [doc = " passed as a single parameter. For example, if tupling is enabled, this"] # [doc = " function:"] # [doc = " ```"] # [doc = " fn f(x: (isize, isize)) {}"] # [doc = " ```"] # [doc = " Can be called as:"] # [doc = " ```ignore UNSOLVED (can this be done in user code?)"] # [doc = " # fn f(x: (isize, isize)) {}"] # [doc = " f(1, 2);"] # [doc = " ```"] # [doc = " Instead of:"] # [doc = " ```"] # [doc = " # fn f(x: (isize, isize)) {}"] # [doc = " f((1, 2));"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq)] enum TupleArgumentsFlag { DontTupleArguments , TupleArguments , }
}