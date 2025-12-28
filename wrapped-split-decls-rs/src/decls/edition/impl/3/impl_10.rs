use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for Edition { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { match s { "2015" => Ok (Edition :: Edition2015) , "2018" => Ok (Edition :: Edition2018) , "2021" => Ok (Edition :: Edition2021) , "2024" => Ok (Edition :: Edition2024) , "future" => Ok (Edition :: EditionFuture) , _ => Err (()) , } } }
}