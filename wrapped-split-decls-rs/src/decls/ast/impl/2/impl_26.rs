use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq < & [Symbol] > for Path { # [inline] fn eq (& self , names : & & [Symbol]) -> bool { self . segments . len () == names . len () && self . segments . iter () . zip (names . iter ()) . all (| (s1 , s2) | s1 == s2) } }