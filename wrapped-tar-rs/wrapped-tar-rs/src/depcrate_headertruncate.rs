// Generated macro for truncate (function)
macro_rules! Depcrate_headertruncate {
() => {
// Module: crate::header
// Provides: {"truncate"}
// Dependencies: {}
fn truncate (slice : & [u8]) -> & [u8] { match slice . iter () . position (| i | * i == 0) { Some (i) => & slice [.. i] , None => slice , } }
};
}
