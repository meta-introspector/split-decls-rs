use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_similarities (tape : & HashMap < String , String > , blocks : & HashMap < String , String >) -> Result < Vec < Similarity > > { let mut similarities = Vec :: new () ; for (tape_name , tape_content) in tape { for (block_name , block_content) in blocks { let similarity_score = calculate_similarity (tape_content , block_content) ; if similarity_score > 0.3 { similarities . push (Similarity { tape_macro : tape_name . clone () , output2_block : block_name . clone () , score : similarity_score , }) ; } } } similarities . sort_by (| a , b | b . score . partial_cmp (& a . score) . unwrap ()) ; Ok (similarities) }
}