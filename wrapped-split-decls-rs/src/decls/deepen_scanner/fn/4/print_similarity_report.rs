use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn print_similarity_report (similarities : & [Similarity]) { println ! ("\n🎯 Similarity Analysis Results:") ; println ! ("═══════════════════════════════") ; if similarities . is_empty () { println ! ("✅ No significant similarities found") ; return ; } for sim in similarities { println ! ("📊 {:.1}% similarity" , sim . score * 100.0) ; println ! ("   📼 Tape: {}" , sim . tape_macro) ; println ! ("   📂 Block: {}" , sim . output2_block) ; println ! () ; } }