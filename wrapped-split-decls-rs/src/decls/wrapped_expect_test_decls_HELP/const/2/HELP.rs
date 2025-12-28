use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const HELP : & str = "
You can update all `expect!` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
" ;
}