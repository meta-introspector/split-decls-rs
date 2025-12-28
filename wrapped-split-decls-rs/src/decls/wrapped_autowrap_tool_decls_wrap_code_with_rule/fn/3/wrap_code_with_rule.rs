use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn wrap_code_with_rule (code : & str , rule : & SplitRule) -> String { let imports = rule . imports . join ("
" ,) ; let code_lines : Vec < & str > = code . lines () . filter (| line | ! line . starts_with ("--") && ! line . contains (".rs:")) . collect () ; format ! (r###"\
prelude! {{ 
    {}

}}

mkdecl! {{ 
    {}

}}
"### , imports , code_lines . join ("
")) }