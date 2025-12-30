// Generated macro for Accumulate (trait)
macro_rules! Depcrate_streamAccumulate {
() => {
// Module: crate::stream
// Provides: {"Accumulate"}
// Dependencies: {}
# [doc = " Abstracts something which can extend an `Extend`."] # [doc = " Used to build modified input slices in `escaped_transform`"] pub trait Accumulate < T > : Sized { # [doc = " Create a new `Extend` of the correct type"] fn initial (capacity : Option < usize >) -> Self ; # [doc = " Accumulate the input into an accumulator"] fn accumulate (& mut self , acc : T) ; }
};
}
