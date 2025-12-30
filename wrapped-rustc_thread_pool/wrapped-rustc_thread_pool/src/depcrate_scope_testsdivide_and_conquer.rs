// Generated macro for divide_and_conquer (function)
macro_rules! Depcrate_scope_testsdivide_and_conquer {
() => {
// Module: crate::scope::tests
// Provides: {"divide_and_conquer"}
// Dependencies: {}
fn divide_and_conquer < 'scope > (scope : & Scope < 'scope > , counter : & 'scope AtomicUsize , size : usize) { if size > 1 { scope . spawn (move | scope | divide_and_conquer (scope , counter , size / 2)) ; scope . spawn (move | scope | divide_and_conquer (scope , counter , size / 2)) ; } else { counter . fetch_add (1 , Ordering :: SeqCst) ; } }
};
}
