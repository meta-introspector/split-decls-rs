use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn check_preservation (tokens : & (String , String) , input_2grams : & HashMap < (String , String) , usize > , output_2grams : & HashMap < (String , String) , usize >) -> bool { let input_count = input_2grams . get (tokens) . unwrap_or (& 0) ; for (output_pair , output_count) in output_2grams { if output_count >= input_count { return true ; } } * input_count > 5 }
}