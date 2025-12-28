use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug)] pub enum ParseResult < F > { Success (NamedMatches) , Failure (F) , Error (Span , Cow < 'static , str >) , ErrorReported (ErrorGuaranteed) , }
}