// Generated macro for impl_82 (impl)
macro_rules! Depcrate_conditionimpl_82 {
() => {
// Module: crate::condition
// Provides: {"impl_82"}
// Dependencies: {}
impl AtomicCondition { pub const DEFAULT : AtomicCondition = AtomicCondition :: from (Condition :: DEFAULT) ; pub const fn from (value : Condition) -> Self { AtomicCondition (AtomicPtr :: new (value . 0 as * mut ())) } pub fn store (& self , cond : Condition) { self . 0 . store (cond . 0 as * mut () , Ordering :: Release) } pub fn read (& self) -> bool { let condition = unsafe { Condition (core :: mem :: transmute (self . 0 . load (Ordering :: Acquire))) } ; condition () } }
};
}
