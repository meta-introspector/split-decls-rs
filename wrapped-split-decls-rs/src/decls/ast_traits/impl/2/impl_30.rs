use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , Tag > From < AstNodeWrapper < Box < T > , Tag > > for AstNodeWrapper < T , Tag > { fn from (value : AstNodeWrapper < Box < T > , Tag >) -> Self { AstNodeWrapper { wrapped : * value . wrapped , tag : value . tag } } }