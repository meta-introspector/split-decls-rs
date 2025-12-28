use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , PartialOrd)] pub enum ExprPrecedence { Jump , Assign , Range , LOr , LAnd , Compare , BitOr , BitXor , BitAnd , Shift , Sum , Product , Cast , Prefix , Unambiguous , }