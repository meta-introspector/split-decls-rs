use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IndexEntryExtendedFlag { is_bit_set ! (is_intent_to_add , IndexEntryExtendedFlag :: INTENT_TO_ADD) ; is_bit_set ! (is_skip_worktree , IndexEntryExtendedFlag :: SKIP_WORKTREE) ; is_bit_set ! (is_up_to_date , IndexEntryExtendedFlag :: UPTODATE) ; }