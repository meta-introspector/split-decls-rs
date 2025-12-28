use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Annotations < usize > for Maxes { fn new (& self , element : usize) -> MaxReached { MaxReached (self . 1 (element)) } fn annotate_scc (& mut self , scc : usize , annotation : MaxReached) { let i = self . 0 . push (annotation) ; assert ! (i == scc) ; } type Ann = MaxReached ; type SccIdx = usize ; }