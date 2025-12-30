// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl < T , U , V > TupleExt for (T , U , V) { type Head = T ; type Tail = V ; fn head (self) -> Self :: Head { self . 0 } fn tail (self) -> Self :: Tail { self . 2 } }
};
}
