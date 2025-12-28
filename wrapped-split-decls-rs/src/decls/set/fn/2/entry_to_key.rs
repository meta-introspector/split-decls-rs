use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Adapter function used to return"] # [doc = " result if SsoHashMap functions into"] # [doc = " result SsoHashSet should return."] # [inline (always)] fn entry_to_key < K , V > ((k , _v) : (K , V)) -> K { k }