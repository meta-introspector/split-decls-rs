// Generated macro for strategy (module)
macro_rules! Depcrate_litstrategy {
() => {
// Module: crate::lit
// Provides: {"strategy"}
// Dependencies: {}
# [cfg (any (test , feature = "proptest-strategies"))] # [doc (hidden)] pub mod strategy { use super :: * ; use proptest :: { prelude :: * , * } ; pub fn var (index : impl Strategy < Value = usize >) -> impl Strategy < Value = Var > { index . prop_map (Var :: from_index) } pub fn lit (index : impl Strategy < Value = usize >) -> impl Strategy < Value = Lit > { (var (index) , bool :: ANY) . prop_map (| (var , polarity) | var . lit (polarity)) } }
};
}
