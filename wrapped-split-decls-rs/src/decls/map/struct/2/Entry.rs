use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A view into a single entry in a map."] pub struct Entry < 'a , K , V > { ssomap : & 'a mut SsoHashMap < K , V > , key : K , }