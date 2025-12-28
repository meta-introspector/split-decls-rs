use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Rustc Eigenmatrix - Mathematical representation of the entire rustc compiler"] # [derive (Debug)] pub struct RustcEigenmatrix { # [doc = " Feature matrix of rustc components"] pub matrix : UrlMatrix , # [doc = " Rustc source analysis"] pub analysis : RustcAnalysis , # [doc = " Eigenform of the compiler"] pub eigenform : Option < CompilerEigenform > , }
}