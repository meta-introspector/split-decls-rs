use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > FromIterator < & 'a SubdiagnosticKind > for KindsStatistics { fn from_iter < T : IntoIterator < Item = & 'a SubdiagnosticKind > > (kinds : T) -> Self { let mut ret = Self { has_multipart_suggestion : false , all_multipart_suggestions : true , has_normal_suggestion : false , all_applicabilities_static : true , } ; for kind in kinds { if let SubdiagnosticKind :: MultipartSuggestion { applicability : None , .. } | SubdiagnosticKind :: Suggestion { applicability : None , .. } = kind { ret . all_applicabilities_static = false ; } if let SubdiagnosticKind :: MultipartSuggestion { .. } = kind { ret . has_multipart_suggestion = true ; } else { ret . all_multipart_suggestions = false ; } if let SubdiagnosticKind :: Suggestion { .. } = kind { ret . has_normal_suggestion = true ; } } ret } }
}