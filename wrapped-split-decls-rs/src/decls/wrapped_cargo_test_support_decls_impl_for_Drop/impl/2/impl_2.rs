use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for Execs { fn drop (& mut self) { if ! self . ran && ! std :: thread :: panicking () { panic ! ("forgot to run this command") ; } } }