// Generated macro for tokenize (function)
macro_rules! Depcratetokenize {
() => {
// Module: crate
// Provides: {"tokenize"}
// Dependencies: {}
fn tokenize (cmd : & str) -> impl Iterator < Item = Result < Token < '_ > > > + '_ { let mut cmd = strip_matches (cmd , "\"") ; iter :: from_fn (move | | { let old_len = cmd . len () ; cmd = cmd . trim_start () ; let joined_to_prev = old_len == cmd . len () ; if cmd . is_empty () { return None ; } let (len , kind) = match next_token (cmd) { Ok (it) => it , Err (err) => { cmd = "" ; return Some (Err (err)) ; } } ; let token = Token { joined_to_prev , text : & cmd [.. len] , kind } ; cmd = & cmd [len ..] ; Some (Ok (token)) }) }
};
}
