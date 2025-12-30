// Generated macro for layer (function)
macro_rules! Depcratelayer {
() => {
// Module: crate
// Provides: {"layer"}
// Dependencies: {}
# [doc = " Construct a journald layer"] # [doc = ""] # [doc = " Fails if the journald socket couldn't be opened."] pub fn layer () -> io :: Result < Layer > { Layer :: new () }
};
}
