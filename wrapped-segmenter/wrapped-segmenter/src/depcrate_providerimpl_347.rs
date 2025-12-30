// Generated macro for impl_347 (impl)
macro_rules! Depcrate_providerimpl_347 {
() => {
// Module: crate::provider
// Provides: {"impl_347"}
// Dependencies: {}
impl zerovec :: ule :: AsULE for BreakState { type ULE = u8 ; fn to_unaligned (self) -> Self :: ULE { match self { BreakState :: Break => 253 , BreakState :: Keep => 255 , BreakState :: NoMatch => 254 , BreakState :: Intermediate (i) => i + 120 , BreakState :: Index (i) => i , } } fn from_unaligned (unaligned : Self :: ULE) -> Self { match unaligned { 253 => BreakState :: Break , 255 => BreakState :: Keep , 254 => BreakState :: NoMatch , i if (120 .. 253) . contains (& i) => BreakState :: Intermediate (i - 120) , i => BreakState :: Index (i) , } } }
};
}
