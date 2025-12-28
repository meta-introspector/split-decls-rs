use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait CompilerHost < C > { fn run_compiler_callbacks (& self , args : Vec < String > , callbacks : & mut C) ; }
}