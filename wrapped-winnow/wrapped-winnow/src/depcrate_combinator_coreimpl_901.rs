// Generated macro for impl_901 (impl)
macro_rules! Depcrate_combinator_coreimpl_901 {
() => {
// Module: crate::combinator::core
// Provides: {"impl_901"}
// Dependencies: {}
impl < F , I , O , E > ParserIterator < F , I , O , E > where F : Parser < I , O , E > , I : Stream , E : ParserError < I > , { # [doc = " Returns the remaining input if parsing was successful, or the error if we encountered an error."] pub fn finish (self) -> Result < (I , ()) , E > { match self . state { State :: Running | State :: Done => Ok ((self . input , ())) , State :: Cut (e) => Err (e) , } } }
};
}
