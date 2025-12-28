use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Fill-in macro expansion result, to allow compilation to continue"] # [doc = " after hitting errors."] # [derive (Copy , Clone)] pub struct DummyResult { guar : Option < ErrorGuaranteed > , span : Span , }