use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AlignFromBytesError { pub fn diag_ident (self) -> & 'static str { match self { Self :: NotPowerOfTwo (_) => "not_power_of_two" , Self :: TooLarge (_) => "too_large" , } } pub fn align (self) -> u64 { let (Self :: NotPowerOfTwo (align) | Self :: TooLarge (align)) = self ; align } }