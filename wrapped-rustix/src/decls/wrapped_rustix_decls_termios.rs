use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, target_os = "horizon", target_os = "vita")))]
#[cfg(feature = "termios")]
#[cfg_attr(docsrs, doc(cfg(feature = "termios")))]
pub mod termios;
