// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl From < usize > for State { fn from (num : usize) -> State { match num { 0 => State :: Idle , 1 => State :: Want , 2 => State :: Give , 3 => State :: Closed , _ => unreachable ! ("unknown state: {}" , num) , } } }
};
}
