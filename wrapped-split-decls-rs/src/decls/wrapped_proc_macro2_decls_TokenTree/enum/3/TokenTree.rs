use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`)."] # [derive (Clone)] pub enum TokenTree { # [doc = " A token stream surrounded by bracket delimiters."] Group (Group) , # [doc = " An identifier."] Ident (Ident) , # [doc = " A single punctuation character (`+`, `,`, `$`, etc.)."] Punct (Punct) , # [doc = " A literal character (`'a'`), string (`\"hello\"`), number (`2.3`), etc."] Literal (Literal) , }
}