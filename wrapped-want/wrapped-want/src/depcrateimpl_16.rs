// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl From < State > for usize { fn from (s : State) -> usize { match s { State :: Idle => 0 , State :: Want => 1 , State :: Give => 2 , State :: Closed => 3 , } } }
};
}
