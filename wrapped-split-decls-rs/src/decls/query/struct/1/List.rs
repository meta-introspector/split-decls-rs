use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A type used to greedily parse another type until the input is empty."] struct List < T > (Vec < T >) ;