use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N : Idx , S : Idx + Ord > Annotations < N > for NoAnnotations < S > { type SccIdx = S ; type Ann = () ; fn new (& self , _element : N) { } fn annotate_scc (& mut self , _scc : S , _annotation : ()) { } }
}