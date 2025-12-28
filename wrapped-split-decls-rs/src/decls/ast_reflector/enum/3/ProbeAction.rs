use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum ProbeAction { Log { message : String } , AddAttribute { attr : String } , WrapFunction { wrapper : String } , InjectCode { code : String , position : InjectionPosition } , Transform { macro_name : String , args : Vec < String > } , Collect { field : String } , Enhance { enhancement_type : String , data : String } , }
}