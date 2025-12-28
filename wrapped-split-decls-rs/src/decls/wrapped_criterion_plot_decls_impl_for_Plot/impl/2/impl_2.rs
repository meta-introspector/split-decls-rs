use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Plot { fn new < S > (data : Matrix , script : & S) -> Plot where S : Script , { Plot { data , script : script . script () , } } fn data (& self) -> & Matrix { & self . data } fn script (& self) -> & str { & self . script } }