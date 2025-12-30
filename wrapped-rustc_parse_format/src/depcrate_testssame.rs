// Generated macro for same (function)
macro_rules! Depcrate_testssame {
() => {
// Module: crate::tests
// Provides: {"same"}
// Dependencies: {}
# [track_caller] fn same (fmt : & 'static str , p : & [Piece < 'static >]) { let parser = Parser :: new (fmt , None , None , false , ParseMode :: Format) ; assert_eq ! (parser . collect ::< Vec < Piece <'static >>> () , p) ; }
};
}
