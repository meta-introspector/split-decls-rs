// Generated macro for test_compile (function)
macro_rules! Depcratetest_compile {
() => {
// Module: crate
// Provides: {"test_compile"}
// Dependencies: {}
pub fn test_compile () { let mut map = Map :: default () ; map . insert (1 , 1) ; map . insert (2 , 4) ; for (_ , _) in map . iter () { } let _map2 = Map :: from_iter (Some ((1 , 1))) ; let mut set = Set :: default () ; set . insert ("a") ; }
};
}
