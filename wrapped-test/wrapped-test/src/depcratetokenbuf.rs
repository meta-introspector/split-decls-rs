// Generated macro for TokenBuf (struct)
macro_rules! DepcrateTokenBuf {
() => {
// Module: crate
// Provides: {"TokenBuf"}
// Dependencies: {}
# [doc = "\nA buffer for collecting test tokens.\n\nThis type shouldn't be used as a general-purpose buffer.\nSee the `sval-buffer` library for that.\n"] # [derive (Default , PartialEq , Debug)] pub struct TokenBuf < 'a > { tokens : Vec < Token < 'a > > , }
};
}
