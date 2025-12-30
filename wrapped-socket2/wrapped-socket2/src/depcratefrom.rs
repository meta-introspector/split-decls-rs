// Generated macro for from (macro)
macro_rules! Depcratefrom {
() => {
// Module: crate
// Provides: {"from"}
// Dependencies: {}
# [doc = " Macro to convert from one network type to another."] macro_rules ! from { ($ from : ty , $ for : ty) => { impl From <$ from > for $ for { fn from (socket : $ from) -> $ for { # [cfg (unix)] unsafe { <$ for >:: from_raw_fd (socket . into_raw_fd ()) } # [cfg (windows)] unsafe { <$ for >:: from_raw_socket (socket . into_raw_socket ()) } } } } ; }
};
}
