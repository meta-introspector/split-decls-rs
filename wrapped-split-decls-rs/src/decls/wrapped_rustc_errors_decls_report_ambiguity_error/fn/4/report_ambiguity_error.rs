use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: report_ambiguity_error");
pub fn report_ambiguity_error < 'a , G : EmissionGuarantee > (diag : & mut Diag < 'a , G > , ambiguity : rustc_lint_defs :: AmbiguityErrorDiag ,) { diag . span_label (ambiguity . label_span , ambiguity . label_msg) ; diag . note (ambiguity . note_msg) ; diag . span_note (ambiguity . b1_span , ambiguity . b1_note_msg) ; for help_msg in ambiguity . b1_help_msgs { diag . help (help_msg) ; } diag . span_note (ambiguity . b2_span , ambiguity . b2_note_msg) ; for help_msg in ambiguity . b2_help_msgs { diag . help (help_msg) ; } }
}