use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < G > std :: fmt :: Debug for DepthFirstSearch < G > where G : DirectedGraph + Successors , { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut f = fmt . debug_set () ; for n in self . visited . iter () { f . entry (& n) ; } f . finish () } }