use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AssignOpKind { pub fn as_str (& self) -> & 'static str { use AssignOpKind :: * ; match self { AddAssign => "+=" , SubAssign => "-=" , MulAssign => "*=" , DivAssign => "/=" , RemAssign => "%=" , BitXorAssign => "^=" , BitAndAssign => "&=" , BitOrAssign => "|=" , ShlAssign => "<<=" , ShrAssign => ">>=" , } } # [doc = " AssignOps are always by value."] pub fn is_by_value (self) -> bool { true } }