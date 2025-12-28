use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns a span encompassing all tokens in the iterator if there is at least one item."] fn iter_span (iter : & TokenStreamIter < '_ >) -> Option < Span > { let mut iter = iter . clone () ; let first_sp = iter . next () ? . span () ; let last_sp = iter . last () . map (TokenTree :: span) . unwrap_or (first_sp) ; let span = first_sp . with_hi (last_sp . hi ()) ; Some (span) }
}