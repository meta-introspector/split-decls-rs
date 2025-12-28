use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Copy > AppendOnlyVec < T > { pub fn new () -> Self { Self { vec : Default :: default () } } pub fn push (& self , val : T) -> usize { let mut v = self . vec . write () ; let n = v . len () ; v . push (val) ; n } pub fn get (& self , i : usize) -> Option < T > { self . vec . read () . get (i) . copied () } pub fn iter_enumerated (& self) -> impl Iterator < Item = (usize , T) > { (0 ..) . map (| i | (i , self . get (i))) . take_while (| (_ , o) | o . is_some ()) . filter_map (| (i , o) | Some ((i , o ?))) } pub fn iter (& self) -> impl Iterator < Item = T > { (0 ..) . map (| i | self . get (i)) . take_while (| o | o . is_some ()) . flatten () } }
}