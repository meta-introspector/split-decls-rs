use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Placeholder for the LLM's resolution."] # [derive (Debug , Clone , Serialize , Deserialize)] pub enum Resolution { Continue , ModifyCode { file : String , line : u32 , column : u32 , new_code : String , } , }