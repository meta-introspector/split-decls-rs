// Generated macro for PResult (type)
macro_rules! DepcratePResult {
() => {
// Module: crate
// Provides: {"PResult"}
// Dependencies: {}
# [doc = " The result of a parser"] pub type PResult < 'a , O > = Result < (Cursor < 'a > , O) , ParseError > ;
};
}
