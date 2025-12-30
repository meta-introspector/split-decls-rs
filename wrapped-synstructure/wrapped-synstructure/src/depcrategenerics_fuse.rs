// Generated macro for generics_fuse (function)
macro_rules! Depcrategenerics_fuse {
() => {
// Module: crate
// Provides: {"generics_fuse"}
// Dependencies: {}
fn generics_fuse (res : & mut Vec < bool > , new : & [bool]) { for (i , & flag) in new . iter () . enumerate () { if i == res . len () { res . push (false) ; } if flag { res [i] = true ; } } }
};
}
