use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Annotations < usize > for MinMaxes { fn new (& self , element : usize) -> MinMaxIn { self . 1 (element) } fn annotate_scc (& mut self , scc : usize , annotation : MinMaxIn) { let i = self . 0 . push (annotation) ; assert ! (i == scc) ; } type Ann = MinMaxIn ; type SccIdx = usize ; }
}