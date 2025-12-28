use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " SHA-1 collision detection hasher state."] # [derive (Clone)] pub struct Sha1 { h : [u32 ; STATE_LEN] , block_len : u64 , detection : Option < DetectionState > , buffer : BlockBuffer < U64 , Eager > , }
}