use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: compress_subtree_to_parent_node");
fn compress_subtree_to_parent_node < J : join :: Join > (input : & [u8] , key : & CVWords , chunk_counter : u64 , flags : u8 , platform : Platform ,) -> [u8 ; BLOCK_LEN] { debug_assert ! (input . len () > CHUNK_LEN) ; let mut cv_array = [0 ; MAX_SIMD_DEGREE_OR_2 * OUT_LEN] ; let mut num_cvs = compress_subtree_wide :: < J > (input , & key , chunk_counter , flags , platform , & mut cv_array) ; debug_assert ! (num_cvs >= 2) ; let mut out_array = [0 ; MAX_SIMD_DEGREE_OR_2 * OUT_LEN / 2] ; while num_cvs > 2 { let cv_slice = & cv_array [.. num_cvs * OUT_LEN] ; num_cvs = compress_parents_parallel (cv_slice , key , flags , platform , & mut out_array) ; cv_array [.. num_cvs * OUT_LEN] . copy_from_slice (& out_array [.. num_cvs * OUT_LEN]) ; } * array_ref ! (cv_array , 0 , 2 * OUT_LEN) }
}