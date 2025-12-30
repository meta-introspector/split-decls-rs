// Generated macro for todo (function)
macro_rules! Depcrate_combinator_coretodo {
() => {
// Module: crate::combinator::core
// Provides: {"todo"}
// Dependencies: {}
# [doc = " A placeholder for a not-yet-implemented [`Parser`]"] # [doc = ""] # [doc = " This is analogous to the [`todo!`] macro and helps with prototyping."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " This will panic when parsing"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::combinator::todo;"] # [doc = ""] # [doc = " fn parser(input: &mut &str) -> ModalResult<u64> {"] # [doc = "     todo(input)"] # [doc = " }"] # [doc = " ```"] # [track_caller] pub fn todo < Input , Output , Error > (input : & mut Input) -> Result < Output , Error > where Input : Stream , Error : ParserError < Input > , { # ! [allow (clippy :: todo)] trace ("todo" , move | _input : & mut Input | { todo ! ("unimplemented parse") }) . parse_next (input) }
};
}
