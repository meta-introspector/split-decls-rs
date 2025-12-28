use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct CompilerEigenform { # [doc = " Principal components of rustc"] pub components : Vec < PrincipalComponent > , # [doc = " Compressed compiler representation"] pub compressed_rustc : Vec < f64 > , # [doc = " Reconstruction capability"] pub fidelity : f64 , }