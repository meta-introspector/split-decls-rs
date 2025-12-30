// Generated macro for panic_fwd (function)
macro_rules! Depcrate_spawn_testspanic_fwd {
() => {
// Module: crate::spawn::tests
// Provides: {"panic_fwd"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_fwd () { let (tx , rx) = channel () ; let tx = Mutex :: new (tx) ; let panic_handler = move | err : Box < dyn Any + Send > | { let tx = tx . lock () . unwrap () ; if let Some (& msg) = err . downcast_ref :: < & str > () { if msg == "Hello, world!" { tx . send (1) . unwrap () ; } else { tx . send (2) . unwrap () ; } } else { tx . send (3) . unwrap () ; } } ; let builder = ThreadPoolBuilder :: new () . panic_handler (panic_handler) ; builder . build () . unwrap () . spawn (move | | panic ! ("Hello, world!")) ; assert_eq ! (1 , rx . recv () . unwrap ()) ; }
};
}
