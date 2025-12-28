use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl RegexMatcher for DummyRegexMatcher { fn new (_re : & str) -> Result < Self , String > { Ok (DummyRegexMatcher) } fn is_match (& self , _text : & str) -> bool { true } fn captures < 't > (& 't self , _text : & 't str) -> Option < Box < dyn RegexCaptures + 't > > { None } }