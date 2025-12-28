use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn emit_delayed_lint (lint : & DelayedLint , tcx : TyCtxt < '_ >) { match lint { DelayedLint :: AttributeParsing (attribute_lint) => { rustc_attr_parsing :: emit_attribute_lint (attribute_lint , tcx) } } }