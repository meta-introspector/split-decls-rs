use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "unstable")]
unsafe impl<T> core::iter::TrustedLen for Drain<'_, T> {}
