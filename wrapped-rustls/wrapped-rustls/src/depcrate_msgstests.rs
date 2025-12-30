// Generated macro for tests (module)
macro_rules! Depcrate_msgstests {
() => {
// Module: crate::msgs
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: codec :: Reader ; use super :: message :: Message ; use crate :: crypto :: cipher :: { OutboundOpaqueMessage , PlainMessage , PrefixedPayload } ; # [test] fn smoketest () { let bytes = include_bytes ! ("../testdata/handshake-test.1.bin") ; let mut r = Reader :: init (bytes) ; while r . any_left () { let m = PlainMessage :: read (& mut r) . unwrap () ; let out = OutboundOpaqueMessage { typ : m . typ , version : m . version , payload : PrefixedPayload :: from (m . payload . bytes ()) , } . encode () ; assert ! (! out . is_empty ()) ; Message :: try_from (m) . unwrap () ; } } }
};
}
