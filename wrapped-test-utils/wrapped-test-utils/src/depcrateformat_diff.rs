// Generated macro for format_diff (function)
macro_rules! Depcrateformat_diff {
() => {
// Module: crate
// Provides: {"format_diff"}
// Dependencies: {}
pub fn format_diff (chunks : Vec < dissimilar :: Chunk < '_ > >) -> String { let mut buf = String :: new () ; for chunk in chunks { let formatted = match chunk { dissimilar :: Chunk :: Equal (text) => text . into () , dissimilar :: Chunk :: Delete (text) => format ! ("\x1b[41m{text}\x1b[0m\x1b[K") , dissimilar :: Chunk :: Insert (text) => format ! ("\x1b[42m{text}\x1b[0m\x1b[K") , } ; buf . push_str (& formatted) ; } buf }
};
}
