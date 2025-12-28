use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn bisect_split < 'a , 'b > (text1 : Range < 'a > , text2 : Range < 'b > , x : usize , y : usize ,) -> Vec < Diff < 'a , 'b > > { let (text1a , text1b) = text1 . split_at (x) ; let (text2a , text2b) = text2 . split_at (y) ; let mut diffs = main (text1a , text2a) . diffs ; diffs . extend (main (text1b , text2b) . diffs) ; diffs }