use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn calculate_complexity_score (field_count : usize , variant_count : usize , nested_depth : u8) -> u8 { let base_score = match field_count { 0 ..= 2 => 1 , 3 ..= 4 => 2 , 5 ..= 6 => 3 , 7 ..= 8 => 4 , 9 ..= 10 => 5 , 11 ..= 12 => 6 , _ => 7 , } ; let variant_bonus = match variant_count { 0 ..= 2 => 0 , 3 ..= 5 => 1 , 6 ..= 10 => 2 , _ => 3 , } ; let depth_bonus = nested_depth . min (3) ; (base_score + variant_bonus + depth_bonus) . min (10) }