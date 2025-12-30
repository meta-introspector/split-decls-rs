// Generated macro for iterator (function)
macro_rules! Depcrate_combinator_coreiterator {
() => {
// Module: crate::combinator::core
// Provides: {"iterator"}
// Dependencies: {}
# [doc = " Repeats the embedded parser, lazily returning the results"] # [doc = ""] # [doc = " Call the iterator's [`ParserIterator::finish`] method to get the remaining input if successful,"] # [doc = " or the error value if we encountered an error."] # [doc = ""] # [doc = " On [`ErrMode::Backtrack`][crate::error::ErrMode::Backtrack], iteration will stop. To instead chain an error up, see [`cut_err`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::{combinator::iterator, ascii::alpha1, combinator::terminated};"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let data = \"abc|defg|hijkl|mnopqr|123\";"] # [doc = " let mut it = iterator(data, terminated(alpha1, \"|\"));"] # [doc = ""] # [doc = " let parsed = it.map(|v| (v, v.len())).collect::<HashMap<_,_>>();"] # [doc = " let res: ModalResult<_> = it.finish();"] # [doc = ""] # [doc = " assert_eq!(parsed, [(\"abc\", 3usize), (\"defg\", 4), (\"hijkl\", 5), (\"mnopqr\", 6)].iter().cloned().collect());"] # [doc = " assert_eq!(res, Ok((\"123\", ())));"] # [doc = " ```"] pub fn iterator < Input , Output , Error , ParseNext > (input : Input , parser : ParseNext ,) -> ParserIterator < ParseNext , Input , Output , Error > where ParseNext : Parser < Input , Output , Error > , Input : Stream , Error : ParserError < Input > , { ParserIterator { parser , input , state : State :: Running , o : Default :: default () , } }
};
}
