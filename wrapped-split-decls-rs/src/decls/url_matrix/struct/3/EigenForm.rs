use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct EigenForm { # [doc = " Eigenvalues (principal components)"] pub eigenvalues : Vec < f64 > , # [doc = " Eigenvectors (basis vectors)"] pub eigenvectors : Vec < Vec < f64 > > , # [doc = " Compressed representation"] pub compressed : Vec < f64 > , # [doc = " Compression ratio"] pub ratio : f64 , }
}