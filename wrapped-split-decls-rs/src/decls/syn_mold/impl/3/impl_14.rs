use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SynUsageVisitor { fn analyze_block_for_syn_usage (& mut self , context : & str , block : & syn :: Block) { let block_str = quote ! (# block) . to_string () ; if block_str . contains ("syn::parse") { self . parse_calls += 1 ; } if block_str . contains ("visit") { self . visit_calls += 1 ; } if block_str . contains ("quote!") { self . generation_calls += 1 ; } if block_str . contains ("fold") || block_str . contains ("transform") { self . transform_calls += 1 ; } } }