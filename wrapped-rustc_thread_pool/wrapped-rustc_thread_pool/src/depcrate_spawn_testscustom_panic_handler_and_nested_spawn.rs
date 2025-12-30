// Generated macro for custom_panic_handler_and_nested_spawn (function)
macro_rules! Depcrate_spawn_testscustom_panic_handler_and_nested_spawn {
() => {
// Module: crate::spawn::tests
// Provides: {"custom_panic_handler_and_nested_spawn"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn custom_panic_handler_and_nested_spawn () { let (tx , rx) = channel () ; let tx = Mutex :: new (tx) ; let panic_handler = move | e | { tx . lock () . unwrap () . send (e) . unwrap () ; } ; const PANICS : usize = 3 ; let builder = ThreadPoolBuilder :: new () . panic_handler (panic_handler) ; builder . build () . unwrap () . spawn (move | | { for _ in 0 .. PANICS { spawn (move | | { panic ! ("Hello, world!") ; }) ; } }) ; for _ in 0 .. PANICS { let error = rx . recv () . unwrap () ; if let Some (& msg) = error . downcast_ref :: < & str > () { assert_eq ! (msg , "Hello, world!") ; } else { panic ! ("did not receive a string from panic handler") ; } } }
};
}
