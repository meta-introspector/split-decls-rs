use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
const E_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8000000B_u32 as _);
