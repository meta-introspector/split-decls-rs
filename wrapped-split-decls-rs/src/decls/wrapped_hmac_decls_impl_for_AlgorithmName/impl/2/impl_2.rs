use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : EagerHash + AlgorithmName > AlgorithmName for HmacReset < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }