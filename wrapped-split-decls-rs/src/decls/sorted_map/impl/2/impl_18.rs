use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K : Debug , V : Debug > Debug for SortedMap < K , V > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . data . iter () . map (| (a , b) | (a , b))) . finish () } }