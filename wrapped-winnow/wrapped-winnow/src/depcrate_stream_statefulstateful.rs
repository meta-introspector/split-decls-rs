// Generated macro for Stateful (struct)
macro_rules! Depcrate_stream_statefulStateful {
() => {
// Module: crate::stream::stateful
// Provides: {"Stateful"}
// Dependencies: {}
# [doc = " Thread global state through your parsers"] # [doc = ""] # [doc = " Use cases"] # [doc = " - Recursion checks"] # [doc = " - Error recovery"] # [doc = " - Debugging"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::cell::Cell;"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::stream::Stateful;"] # [doc = " # use winnow::ascii::alpha1;"] # [doc = " # type Error = ();"] # [doc = ""] # [doc = " #[derive(Debug)]"] # [doc = " struct State<'s>(&'s mut u32);"] # [doc = ""] # [doc = " impl<'s> State<'s> {"] # [doc = "     fn count(&mut self) {"] # [doc = "         *self.0 += 1;"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " type Stream<'is> = Stateful<&'is str, State<'is>>;"] # [doc = ""] # [doc = " fn word<'s>(i: &mut Stream<'s>) -> ModalResult<&'s str> {"] # [doc = "   i.state.count();"] # [doc = "   alpha1.parse_next(i)"] # [doc = " }"] # [doc = ""] # [doc = " let data = \"Hello\";"] # [doc = " let mut state = 0;"] # [doc = " let input = Stream { input: data, state: State(&mut state) };"] # [doc = " let output = word.parse(input).unwrap();"] # [doc = " assert_eq!(state, 1);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq)] # [doc (alias = "LocatingSliceSpan")] pub struct Stateful < I , S > { # [doc = " Inner input being wrapped in state"] pub input : I , # [doc = " User-provided state"] pub state : S , }
};
}
