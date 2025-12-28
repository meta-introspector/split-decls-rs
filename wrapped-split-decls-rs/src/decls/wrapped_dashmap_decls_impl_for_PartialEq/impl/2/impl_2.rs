use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , K : 'a + Eq + Hash , V : 'a + PartialEq , S : BuildHasher + Clone > PartialEq for DashMap < K , V , S > { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . all (| r | { other . get (r . key ()) . map_or (false , | ro | r . value () == ro . value ()) }) } }