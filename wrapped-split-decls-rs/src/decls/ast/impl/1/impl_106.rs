use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Box < Expr > > for Expr { fn from (value : Box < Expr >) -> Self { * value } }