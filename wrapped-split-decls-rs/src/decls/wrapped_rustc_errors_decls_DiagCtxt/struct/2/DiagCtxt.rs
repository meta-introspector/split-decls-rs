use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A `DiagCtxt` deals with errors and other compiler output."] # [doc = " Certain errors (fatal, bug, unimpl) may cause immediate exit,"] # [doc = " others log errors for later reporting."] pub struct DiagCtxt { inner : Lock < DiagCtxtInner > , }