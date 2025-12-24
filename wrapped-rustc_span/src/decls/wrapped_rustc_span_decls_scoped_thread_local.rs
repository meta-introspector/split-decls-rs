use serde::{Deserialize, Serialize};
use std::collections::HashMap;
scoped_tls::scoped_thread_local!(static SESSION_GLOBALS : SessionGlobals);
