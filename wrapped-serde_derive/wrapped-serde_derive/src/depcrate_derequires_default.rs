// Generated macro for requires_default (function)
macro_rules! Depcrate_derequires_default {
() => {
// Module: crate::de
// Provides: {"requires_default"}
// Dependencies: {}
fn requires_default (field : & attr :: Field , _variant : Option < & attr :: Variant >) -> bool { if let attr :: Default :: Default = * field . default () { true } else { false } }
};
}
