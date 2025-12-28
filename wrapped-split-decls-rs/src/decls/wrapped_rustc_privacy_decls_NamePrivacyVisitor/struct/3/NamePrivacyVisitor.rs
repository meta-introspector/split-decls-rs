use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Name privacy visitor, checks privacy and reports violations."] # [doc = ""] # [doc = " Most of name privacy checks are performed during the main resolution phase,"] # [doc = " or later in type checking when field accesses and associated items are resolved."] # [doc = " This pass performs remaining checks for fields in struct expressions and patterns."] struct NamePrivacyVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , maybe_typeck_results : Option < & 'tcx ty :: TypeckResults < 'tcx > > , }