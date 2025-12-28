use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum Leaf < S > { Literal (Literal < S >) , Punct (Punct < S >) , Ident (Ident < S >) , }