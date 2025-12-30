// Generated macro for impl_18 (impl)
macro_rules! Depcrate_iterimpl_18 {
() => {
// Module: crate::iter
// Provides: {"impl_18"}
// Dependencies: {}
impl < S : Copy + fmt :: Debug > fmt :: Debug for TtElement < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Leaf (leaf) => f . debug_tuple ("Leaf") . field (leaf) . finish () , Self :: Subtree (subtree , inner) => { f . debug_tuple ("Subtree") . field (subtree) . field (inner) . finish () } } } }
};
}
