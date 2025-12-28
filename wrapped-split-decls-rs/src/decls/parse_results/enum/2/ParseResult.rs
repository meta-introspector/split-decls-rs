use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug)] pub enum ParseResult < F > { Success (NamedMatches) , Failure (F) , Error (Span , Cow < 'static , str >) , ErrorReported (ErrorGuaranteed) , }