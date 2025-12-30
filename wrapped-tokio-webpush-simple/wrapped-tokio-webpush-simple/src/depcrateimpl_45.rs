// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < S1 , S2 > StreamNext < S1 , S2 > where S1 : Stream , S2 : Stream < Error = S1 :: Error > , { fn new (s1 : S1 , s2 : S2) -> StreamNext < S1 , S2 > { StreamNext { left : Some (s1) , right : Some (s2) , } } }
};
}
