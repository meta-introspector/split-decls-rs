// Generated macro for Message (enum)
macro_rules! DepcrateMessage {
() => {
// Module: crate
// Provides: {"Message"}
// Dependencies: {}
enum Message { Enter (f64 , Callsite , Option < u64 >) , Event (f64 , Callsite) , Exit (f64 , Callsite , Option < u64 >) , NewThread (usize , String) , Flush , Drop , StartNew (Option < Box < dyn Write + Send > >) , }
};
}
