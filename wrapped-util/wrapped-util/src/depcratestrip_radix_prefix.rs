// Generated macro for strip_radix_prefix (function)
macro_rules! Depcratestrip_radix_prefix {
() => {
// Module: crate
// Provides: {"strip_radix_prefix"}
// Dependencies: {}
fn strip_radix_prefix (s : & str , radix : u32) -> & str { if radix == 16 { s . strip_prefix ("0x") . unwrap () } else if radix == 2 { s . strip_prefix ("0b") . unwrap () } else { s } }
};
}
