use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: diff");
pub fn diff < 'a > (text1 : & 'a str , text2 : & 'a str) -> Vec < Chunk < 'a > > { let chars1 : Vec < char > = text1 . chars () . collect () ; let chars2 : Vec < char > = text2 . chars () . collect () ; let range1 = Range :: new (& chars1 , ..) ; let range2 = Range :: new (& chars2 , ..) ; let mut solution = main (range1 , range2) ; cleanup_char_boundary (& mut solution) ; cleanup_semantic (& mut solution) ; cleanup_merge (& mut solution) ; let mut chunks = Vec :: new () ; let mut pos1 = 0 ; let mut pos2 = 0 ; for diff in solution . diffs { chunks . push (match diff { Diff :: Equal (range , _) => { let len = range . len_bytes () ; let chunk = Chunk :: Equal (& text1 [pos1 .. pos1 + len]) ; pos1 += len ; pos2 += len ; chunk } Diff :: Delete (range) => { let len = range . len_bytes () ; let chunk = Chunk :: Delete (& text1 [pos1 .. pos1 + len]) ; pos1 += len ; chunk } Diff :: Insert (range) => { let len = range . len_bytes () ; let chunk = Chunk :: Insert (& text2 [pos2 .. pos2 + len]) ; pos2 += len ; chunk } }) ; } chunks }
}