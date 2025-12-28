use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MergeAnalysis { is_bit_set ! (is_none , MergeAnalysis :: ANALYSIS_NONE) ; is_bit_set ! (is_normal , MergeAnalysis :: ANALYSIS_NORMAL) ; is_bit_set ! (is_up_to_date , MergeAnalysis :: ANALYSIS_UP_TO_DATE) ; is_bit_set ! (is_fast_forward , MergeAnalysis :: ANALYSIS_FASTFORWARD) ; is_bit_set ! (is_unborn , MergeAnalysis :: ANALYSIS_UNBORN) ; }
}