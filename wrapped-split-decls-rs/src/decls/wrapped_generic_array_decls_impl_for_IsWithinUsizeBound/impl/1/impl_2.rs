use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N > IsWithinUsizeBound for N where N : typenum :: IsLess < MaxArrayLengthP1 , Output = typenum :: consts :: True > { }