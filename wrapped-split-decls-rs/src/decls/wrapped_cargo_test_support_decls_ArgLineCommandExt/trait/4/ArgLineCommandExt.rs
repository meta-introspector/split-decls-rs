use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Add a list of arguments as a line"] pub trait ArgLineCommandExt : Sized { fn arg_line (mut self , s : & str) -> Self { for mut arg in s . split_whitespace () { if (arg . starts_with ('"') && arg . ends_with ('"')) || (arg . starts_with ('\'') && arg . ends_with ('\'')) { arg = & arg [1 .. (arg . len () - 1) . max (1)] ; } else if arg . contains (& ['"' , '\''] [..]) { panic ! ("shell-style argument parsing is not supported") } self = self . arg (arg) ; } self } fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self ; }
}