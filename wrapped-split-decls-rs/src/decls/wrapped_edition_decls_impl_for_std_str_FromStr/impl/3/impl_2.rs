use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: str :: FromStr for Edition { type Err = ParseEditionError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let res = match s { "2015" => Edition :: Edition2015 , "2018" => Edition :: Edition2018 , "2021" => Edition :: Edition2021 , "2024" => Edition :: Edition2024 , _ => { return Err (ParseEditionError { invalid_input : s . to_owned () , }) ; } } ; Ok (res) } }