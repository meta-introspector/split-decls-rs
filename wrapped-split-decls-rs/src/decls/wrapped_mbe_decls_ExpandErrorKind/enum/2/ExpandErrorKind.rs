use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , PartialEq , Eq , Clone , Hash)] pub enum ExpandErrorKind { BindingError (Box < Box < str > >) , UnresolvedBinding (Box < Box < str > >) , LeftoverTokens , LimitExceeded , NoMatchingRule , UnexpectedToken , }