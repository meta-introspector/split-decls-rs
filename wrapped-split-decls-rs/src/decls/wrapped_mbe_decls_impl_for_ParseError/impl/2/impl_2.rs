use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ParseError { fn expected (e : & str) -> ParseError { ParseError :: Expected (e . into ()) } fn unexpected (e : & str) -> ParseError { ParseError :: UnexpectedToken (e . into ()) } }