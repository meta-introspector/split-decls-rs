use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize)] struct OntologySummary { total_files_analyzed : usize , ngram_analyses : Vec < NGramAnalysisSummary > , core_patterns : Vec < CorePattern > , emoji_distribution : HashMap < String , usize > , }
}