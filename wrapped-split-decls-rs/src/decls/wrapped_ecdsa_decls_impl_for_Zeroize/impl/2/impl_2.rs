use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < C : EcdsaCurve > Zeroize for Signature < C > { fn zeroize (& mut self) { self . r = ScalarValue :: ONE ; self . s = ScalarValue :: ONE ; } }