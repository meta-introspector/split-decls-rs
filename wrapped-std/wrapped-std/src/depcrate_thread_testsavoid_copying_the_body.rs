// Generated macro for avoid_copying_the_body (function)
macro_rules! Depcrate_thread_testsavoid_copying_the_body {
() => {
// Module: crate::thread::tests
// Provides: {"avoid_copying_the_body"}
// Dependencies: {}
fn avoid_copying_the_body < F > (spawnfn : F) where F : FnOnce (Box < dyn Fn () + Send >) , { let (tx , rx) = channel () ; let x : Box < _ > = Box :: new (1) ; let x_in_parent = (& * x) as * const i32 as usize ; spawnfn (Box :: new (move | | { let x_in_child = (& * x) as * const i32 as usize ; tx . send (x_in_child) . unwrap () ; })) ; let x_in_child = rx . recv () . unwrap () ; assert_eq ! (x_in_parent , x_in_child) ; }
};
}
