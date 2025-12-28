use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum AbstractionContent { # [doc = " Level 0, 8, 16...: Concrete code (0-dimensional)"] Concrete (TokenStream) , # [doc = " Level 1, 9, 17...: Pattern (1-dimensional bundle)"] Pattern { template : String , fiber_dim : usize , } , # [doc = " Level 2, 10, 18...: Meta-pattern (back to 0-dimensional!)"] MetaPattern { meta_structure : String , } , # [doc = " Level 3, 11, 19...: Meta-meta-pattern (1-dimensional again)"] MetaMetaPattern { structure : String , fiber_dim : usize , } , # [doc = " Level 4, 12, 20...: Quaternionic (4-dimensional symmetry)"] Quaternionic { real_part : String , imag_parts : [String ; 3] , } , # [doc = " Level 5-7: Dual structures (descent)"] Dual { level : usize , base : Box < AbstractionContent > , } , }
}